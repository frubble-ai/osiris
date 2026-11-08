//! A connection's trigger: a subscription to something happening at the provider, whose
//! deliveries osiris publishes as `trigger.fired` events.

use crate::TriggerStatus;
use osiris_common::secret::Secret;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct Trigger {
    /// `tr_…`.
    pub id: String,
    pub connected_account_id: String,
    pub provider: String,
    /// What it watches: one of its provider's kinds, `<provider>.<kind>`.
    pub kind: String,
    /// The provider's own configuration for it, which the provider validated.
    pub config: serde_json::Value,
    /// What the provider signs deliveries with; never answered.
    pub secret: Secret,
    pub status: TriggerStatus,
    /// The provider's id of what osiris created there, such as a webhook's id.
    pub subscription_id: Option<String>,
    /// What a trigger that checks on a schedule saw last.
    pub checkpoint: Option<serde_json::Value>,
    pub last_error: Option<String>,
    pub poll_task: Option<String>,
    pub poll_task_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
