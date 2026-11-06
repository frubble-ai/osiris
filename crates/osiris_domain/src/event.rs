//! Stateless events for clients: what happened and to which connection, never its data. The
//! client fetches the connection for its current state.

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
}

impl EventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConnectionActive => "connection.active",
            Self::ConnectionExpired => "connection.expired",
            Self::ConnectionRevoked => "connection.revoked",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "connection.active" => Some(Self::ConnectionActive),
            "connection.expired" => Some(Self::ConnectionExpired),
            "connection.revoked" => Some(Self::ConnectionRevoked),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// `ev_…`, the same every time it is sent, so a client can drop repeats.
    pub id: String,
    pub event_type: EventType,
    pub connected_account_id: String,
    pub occurred_at: OffsetDateTime,
}

impl Event {
    /// Creates an event about a connection, dated now.
    pub fn new(event_type: EventType, connected_account_id: &str) -> Self {
        Self {
            id: osiris_common::ids::id("ev"),
            event_type,
            connected_account_id: connected_account_id.to_owned(),
            occurred_at: OffsetDateTime::now_utc(),
        }
    }
}
