//! `[refresh]`: when a connection's refresh task runs. When a refresh fails, the task scheduler's
//! own retry policy decides when to try again, not osiris.

use super::seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct RefreshSettings {
    /// How long before an access token expires it is refreshed, both by its task and when an
    /// action uses it.
    pub margin_seconds: u64,
    /// The shortest delay before a refresh task runs.
    pub min_lead_seconds: u64,
    /// The longest delay before a refresh task runs. A token that does not expire within this
    /// time is refreshed anyway after it, which keeps its refresh token valid. Must be under the
    /// chosen task scheduler's own limit (30 days for Cloud Tasks), checked at startup.
    pub keepalive_seconds: u64,
}

impl RefreshSettings {
    pub fn margin(&self) -> time::Duration {
        seconds(self.margin_seconds)
    }

    pub fn min_lead(&self) -> time::Duration {
        seconds(self.min_lead_seconds)
    }

    pub fn keepalive(&self) -> time::Duration {
        seconds(self.keepalive_seconds)
    }
}

impl Default for RefreshSettings {
    fn default() -> Self {
        Self {
            margin_seconds: 300,
            min_lead_seconds: 30,
            // 29 days: under Cloud Tasks' 30-day limit, and well within the six months after
            // which GitHub drops an unused refresh token.
            keepalive_seconds: 2_505_600,
        }
    }
}
