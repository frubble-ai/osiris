//! What a webhook delivery means for a GitHub trigger: refused unless signed with the trigger's
//! secret; a ping or an event it doesn't watch ignored; for `github.on_repo_event`, the event
//! itself; for `github.watch_pull_request`, a reason to check its pull request now. A body
//! longer than an event carries is never parsed: its payload is noted by its size, and a watch
//! checks its pull request without reading which one the delivery is about.

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
    max_data_bytes: usize,
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
    let too_large = body.len() > max_data_bytes;
    let read = || serde_json::from_slice::<Value>(body).ok();
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
            let payload = if too_large {
                json!({ "omitted": true, "bytes": body.len() })
            } else {
                let Some(payload) = read() else {
                    return TriggerReceipt::Ignored("the delivery's body is not JSON".into());
                };
                payload
            };
            TriggerReceipt::Fired {
                data: json!({ "event": event, "delivery_id": delivery_id, "payload": payload }),
                delivery_id,
            }
        }
        WATCH_PULL_REQUEST => {
            let Ok(config) = WatchPullRequestConfig::read(config) else {
                return TriggerReceipt::Ignored("the trigger's config no longer reads".into());
            };
            if event != "pull_request" {
                return TriggerReceipt::Ignored("a delivery about another pull request".into());
            }
            // Too large to read which pull request it is about: a check costs one request.
            if too_large {
                return TriggerReceipt::Check;
            }
            let number = read().and_then(|payload| payload.get("number").and_then(Value::as_u64));
            if number != Some(config.number) {
                return TriggerReceipt::Ignored("a delivery about another pull request".into());
            }
            TriggerReceipt::Check
        }
        _ => TriggerReceipt::Ignored(format!("github has no trigger {kind:?}")),
    }
}
