//! What a `trigger.fired` event carries: which trigger fired, and what happened at the provider.
//! Unlike a connection's events, which name the connection only, a trigger's event carries the
//! provider's own account of what happened, since that is the event.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggerFired {
    /// `tr_…`.
    pub trigger_id: String,
    /// The trigger's kind, such as `github.on_repo_event`.
    pub trigger: String,
    /// What happened, in the provider's words and the trigger kind's shape.
    pub data: serde_json::Value,
}
