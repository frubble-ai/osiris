//! What a webhook delivery means for a GitHub trigger: refused unless signed with the trigger's
//! secret; a ping or an event it doesn't watch ignored; for `github.on_repo_event`, the event
//! itself; for `github.watch_pull_request`, a reason to check its pull request now.

use super::{
    ON_REPO_EVENT, RepoEventConfig, WATCH_PULL_REQUEST, WatchPullRequestConfig,
    verify_signature::verify_signature,
};
use crate::TriggerReceipt;
use reqwest::header::HeaderMap;
use serde_json::{Value, json};

/// What `body` (raw, with `headers`) means for the `kind` trigger configured by `config`.
pub fn receive_delivery(
    kind: &str,
    config: &Value,
    headers: &HeaderMap,
    body: &[u8],
    secret: &str,
) -> TriggerReceipt {
    let header = |name: &str| headers.get(name).and_then(|value| value.to_str().ok());
    if !verify_signature(secret, body, header("x-hub-signature-256")) {
        return TriggerReceipt::Unsigned;
    }
    let Some(event) = header("x-github-event")
        .map(str::trim)
        .filter(|event| !event.is_empty())
    else {
        return TriggerReceipt::Ignored("the delivery names no GitHub event".into());
    };
    if event == "ping" {
        return TriggerReceipt::Ignored("GitHub's ping, sent when the webhook is created".into());
    }
    let Ok(payload) = serde_json::from_slice::<Value>(body) else {
        return TriggerReceipt::Ignored("the delivery's body is not JSON".into());
    };
    let delivery_id = header("x-github-delivery").map(str::to_owned);
    match kind {
        ON_REPO_EVENT => {
            let Ok(config) = RepoEventConfig::read(config) else {
                return TriggerReceipt::Ignored("the trigger's config no longer reads".into());
            };
            if !config
                .events
                .iter()
                .any(|watched| watched == "*" || watched == event)
            {
                return TriggerReceipt::Ignored(format!("the trigger doesn't watch {event:?}"));
            }
            TriggerReceipt::Fired {
                data: json!({ "event": event, "delivery_id": delivery_id, "payload": payload }),
                delivery_id,
            }
        }
        WATCH_PULL_REQUEST => {
            let Ok(config) = WatchPullRequestConfig::read(config) else {
                return TriggerReceipt::Ignored("the trigger's config no longer reads".into());
            };
            if event != "pull_request"
                || payload.get("number").and_then(Value::as_u64) != Some(config.number)
            {
                return TriggerReceipt::Ignored("a delivery about another pull request".into());
            }
            TriggerReceipt::Check
        }
        _ => TriggerReceipt::Ignored(format!("github has no trigger {kind:?}")),
    }
}
