//! `events`: stateless events for clients. The type and the connection's id, never its data: the
//! client fetches the connection for its current state.

use osiris_domain::{Event, EventType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventMessage {
    /// `ev_…`, the same each time the event is delivered, so a client can drop repeats.
    pub event_id: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub connected_account_id: String,
}

impl From<&Event> for EventMessage {
    fn from(event: &Event) -> Self {
        Self {
            event_id: event.id.clone(),
            event_type: event.event_type,
            connected_account_id: event.connected_account_id.clone(),
        }
    }
}
