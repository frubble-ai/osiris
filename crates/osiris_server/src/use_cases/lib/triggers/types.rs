//! What goes into the trigger operations.

use osiris_common::secret::Secret;
use osiris_domain::TriggerStatus;
use time::OffsetDateTime;

/// What a new trigger is made of, once subscribed at the provider.
pub struct NewTrigger {
    pub id: String,
    pub connected_account_id: String,
    pub kind: String,
    pub config: serde_json::Value,
    pub secret: Secret,
    pub status: TriggerStatus,
    pub subscription_id: Option<String>,
    pub checkpoint: Option<serde_json::Value>,
    pub last_error: Option<String>,
    pub poll_task: Option<String>,
    pub poll_task_at: Option<OffsetDateTime>,
}

/// A trigger's state after a check or a delivery: written whole.
pub struct TriggerState {
    pub status: TriggerStatus,
    pub checkpoint: Option<serde_json::Value>,
    pub last_error: Option<String>,
    pub poll_task: Option<String>,
    pub poll_task_at: Option<OffsetDateTime>,
}
