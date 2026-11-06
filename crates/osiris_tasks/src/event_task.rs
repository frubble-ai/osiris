//! The payload of an `event.publish` task: an event to publish again. The task carries the whole
//! event, so publishing it does not need the database.

use osiris_domain::{Event, EventType};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventTask {
    pub event_id: String,
    pub event_type: EventType,
    pub connected_account_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub occurred_at: OffsetDateTime,
}

impl From<&Event> for EventTask {
    fn from(event: &Event) -> Self {
        Self {
            event_id: event.id.clone(),
            event_type: event.event_type,
            connected_account_id: event.connected_account_id.clone(),
            occurred_at: event.occurred_at,
        }
    }
}

impl From<&EventTask> for Event {
    fn from(task: &EventTask) -> Self {
        Self {
            id: task.event_id.clone(),
            event_type: task.event_type,
            connected_account_id: task.connected_account_id.clone(),
            occurred_at: task.occurred_at,
        }
    }
}
