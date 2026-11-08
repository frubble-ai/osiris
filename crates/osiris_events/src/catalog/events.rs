//! `events`: events for clients. A connection's events carry its id, never its data: the client
//! fetches the connection for its current state. A trigger's (`trigger.fired`) also names the
//! trigger and carries what happened at the provider.

use osiris_domain::{Event, EventType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    /// `ev_…`, the same each time the event is delivered, so a client can drop repeats.
    pub event_id: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub connected_account_id: String,
    /// `tr_…`: the trigger that fired, for `trigger.fired`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// The trigger's kind, such as `github.on_repo_event`, for `trigger.fired`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    /// What happened at the provider, for `trigger.fired`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl From<&Event> for EventMessage {
    fn from(event: &Event) -> Self {
        Self {
            event_id: event.id.clone(),
            event_type: event.event_type,
            connected_account_id: event.connected_account_id.clone(),
            trigger_id: event.fired.as_ref().map(|fired| fired.trigger_id.clone()),
            trigger: event.fired.as_ref().map(|fired| fired.trigger.clone()),
            data: event.fired.as_ref().map(|fired| fired.data.clone()),
        }
    }
}
