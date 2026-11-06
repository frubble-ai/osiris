//! Why a refresh task was scheduled.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefreshReason {
    /// The access token is about to expire. The task runs a few minutes before it does (the
    /// `[refresh] margin`).
    Expiry,
    /// The access token does not expire, or not within the next 29 days. The task refreshes it
    /// anyway after 29 days, so the refresh token stays valid and the task stays within the
    /// 30-day limit on how far ahead a task can be scheduled.
    Keepalive,
    /// An operator asked for an immediate refresh ("refresh now").
    Retry,
}

impl RefreshReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expiry => "expiry",
            Self::Keepalive => "keepalive",
            Self::Retry => "retry",
        }
    }
}
