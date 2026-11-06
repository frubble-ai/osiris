//! What every message travels in: a version, its own id and when it happened.

use crate::Message;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The envelope's format; a change a subscriber must know about moves it.
pub const VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub version: u32,
    /// `msg_…`.
    pub id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub occurred_at: OffsetDateTime,
    #[serde(flatten)]
    pub message: Message,
}

impl Envelope {
    pub fn wrap(message: Message) -> Self {
        Self {
            version: VERSION,
            id: osiris_common::ids::id("msg"),
            occurred_at: OffsetDateTime::now_utc(),
            message,
        }
    }
}
