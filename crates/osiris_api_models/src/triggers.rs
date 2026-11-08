//! `/v1/triggers`: a connection's subscriptions to something happening at its provider, each
//! delivery published as a `trigger.fired` event.

use osiris_domain::TriggerStatus;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use time::OffsetDateTime;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateTrigger {
    /// `ca_…`: the connection the trigger watches through.
    pub connected_account_id: String,
    /// What it watches: `github.on_repo_event` (a repository's webhook events) or
    /// `github.watch_pull_request` (one pull request's changes). An open string: more kinds may
    /// come.
    pub trigger: String,
    /// The kind's own configuration, which its provider validates. `github.on_repo_event`:
    /// `{"owner", "repo", "events": ["push", …] or ["*"], "insecure_ssl"?}`.
    /// `github.watch_pull_request`: `{"owner", "repo", "number"}`.
    #[schema(value_type = Object)]
    pub config: Map<String, Value>,
}

/// A trigger, without its secret.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Trigger {
    /// `tr_…`.
    pub id: String,
    pub connected_account_id: String,
    pub provider: String,
    /// Its kind, such as `github.on_repo_event`.
    pub trigger: String,
    #[schema(value_type = Object)]
    pub config: Value,
    /// `active` (its deliveries are published) or `failed` (the provider refused it; create it
    /// again). An open string.
    #[schema(value_type = String, example = "active")]
    pub status: TriggerStatus,
    /// The provider's id of what osiris created there: a GitHub repository webhook's id.
    pub subscription_id: Option<String>,
    /// Why it failed.
    pub last_error: Option<String>,
    /// When it next checks the provider, for a kind that checks on a schedule.
    #[serde(with = "time::serde::rfc3339::option")]
    #[schema(value_type = Option<String>, format = DateTime)]
    pub next_check_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: OffsetDateTime,
}

impl From<osiris_domain::Trigger> for Trigger {
    fn from(trigger: osiris_domain::Trigger) -> Self {
        Self {
            id: trigger.id,
            connected_account_id: trigger.connected_account_id,
            provider: trigger.provider,
            trigger: trigger.kind,
            config: trigger.config,
            status: trigger.status,
            subscription_id: trigger.subscription_id,
            last_error: trigger.last_error,
            next_check_at: trigger.poll_task_at,
            created_at: trigger.created_at,
            updated_at: trigger.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TriggerFilter {
    /// Only this connection's triggers.
    pub connected_account_id: Option<String>,
    /// A batch's `next_cursor`: the batch after it.
    pub after: Option<String>,
    /// A batch's `previous_cursor`: the batch before it.
    pub before: Option<String>,
    pub limit: Option<i64>,
}
