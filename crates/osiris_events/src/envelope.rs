//! What every message travels in: a version, its own id, when it happened, and the API release
//! its body is written in.

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
    /// The API release the message is written in, such as `2026-11-04.anubis`: the instance's
    /// pinned one. Also sent as the `osiris_version` attribute.
    pub osiris_version: String,
    #[serde(flatten)]
    pub message: Message,
}

impl Envelope {
    /// `message` in a new envelope: a fresh id, now, and the release it is written in.
    pub fn wrap(message: Message, osiris_version: &str) -> Self {
        Self {
            version: VERSION,
            id: osiris_common::ids::id("msg"),
            occurred_at: OffsetDateTime::now_utc(),
            osiris_version: osiris_version.to_owned(),
            message,
        }
    }
}
