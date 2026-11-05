//! `[idempotency]`: how long the first answer to a key is replayed. After that, the key is treated
//! as unused.

use super::seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct IdempotencySettings {
    pub retention_seconds: u64,
    /// The most expired keys deleted each time a new key is written.
    pub purge_batch: i64,
}

impl IdempotencySettings {
    pub fn retention(&self) -> time::Duration {
        seconds(self.retention_seconds)
    }
}

impl Default for IdempotencySettings {
    fn default() -> Self {
        Self {
            retention_seconds: 86_400,
            purge_batch: 100,
        }
    }
}
