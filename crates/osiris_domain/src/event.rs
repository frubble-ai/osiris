//! Events for clients: what happened and to which connection. A connection's events are stateless,
//! never its data (the client fetches the connection for its current state); a trigger's event
//! (`trigger.fired`) carries what happened at the provider, since that is the event.

use crate::TriggerFired;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// The consent finished: the connection is live.
    #[serde(rename = "connection.active")]
    ConnectionActive,
    /// The provider refused a refresh for good: reconnect.
    #[serde(rename = "connection.expired")]
    ConnectionExpired,
    /// The grant is gone at the provider (learned on use): reconnect.
    #[serde(rename = "connection.revoked")]
    ConnectionRevoked,
    /// A trigger saw something happen at the provider.
    #[serde(rename = "trigger.fired")]
    TriggerFired,
}

impl EventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConnectionActive => "connection.active",
            Self::ConnectionExpired => "connection.expired",
            Self::ConnectionRevoked => "connection.revoked",
            Self::TriggerFired => "trigger.fired",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "connection.active" => Some(Self::ConnectionActive),
            "connection.expired" => Some(Self::ConnectionExpired),
            "connection.revoked" => Some(Self::ConnectionRevoked),
            "trigger.fired" => Some(Self::TriggerFired),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    /// `ev_…`, the same every time it is sent, so a client can drop repeats.
    pub id: String,
    pub event_type: EventType,
    pub connected_account_id: String,
    pub occurred_at: OffsetDateTime,
    /// The trigger that fired and what it saw, for a `trigger.fired` event.
    pub fired: Option<TriggerFired>,
}

impl Event {
    /// Creates an event about a connection, dated now.
    pub fn new(event_type: EventType, connected_account_id: &str) -> Self {
        Self {
            id: osiris_common::ids::id("ev"),
            event_type,
            connected_account_id: connected_account_id.to_owned(),
            occurred_at: OffsetDateTime::now_utc(),
            fired: None,
        }
    }

    /// A `trigger.fired` event, its id `id` so the same delivery is the same event each time it
    /// is published.
    pub fn trigger_fired(id: String, connected_account_id: &str, fired: TriggerFired) -> Self {
        Self {
            id,
            event_type: EventType::TriggerFired,
            connected_account_id: connected_account_id.to_owned(),
            occurred_at: OffsetDateTime::now_utc(),
            fired: Some(fired),
        }
    }
}
