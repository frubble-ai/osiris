//! `[server]`: the API's request limits, and how long a process takes to stop. Where a process
//! listens is not a setting: Cloud Run's `PORT` (see `config/listen.rs`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ServerSettings {
    pub request_timeout_seconds: u64,
    pub max_body_bytes: u64,
    /// After SIGTERM, how long requests already started may take to finish before the process
    /// exits. Under Cloud Run's grace period.
    pub shutdown_grace_seconds: u64,
}

impl ServerSettings {
    pub fn request_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.request_timeout_seconds)
    }

    pub fn max_body(&self) -> usize {
        usize::try_from(self.max_body_bytes).unwrap_or(usize::MAX)
    }

    pub fn shutdown_grace(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.shutdown_grace_seconds)
    }
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            request_timeout_seconds: 30,
            max_body_bytes: 1_048_576,
            shutdown_grace_seconds: 8,
        }
    }
}
