//! `[consent]`: how long a provider's consent stays open. When that time runs out, a scheduled
//! task marks the consent as failed if nobody finished it.

use super::seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConsentSettings {
    pub ttl_seconds: u64,
    pub state_length: usize,
}

impl ConsentSettings {
    pub fn ttl(&self) -> time::Duration {
        seconds(self.ttl_seconds)
    }
}

impl Default for ConsentSettings {
    fn default() -> Self {
        Self {
            ttl_seconds: 900,
            state_length: 40,
        }
    }
}
