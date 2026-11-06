//! `[http]`: calls to providers and Pub/Sub.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct HttpSettings {
    pub connect_timeout_seconds: u64,
    pub timeout_seconds: u64,
}

impl HttpSettings {
    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_seconds)
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }

    /// The HTTP client for providers and Google's keys: bounded, so a slow peer can't hold a
    /// request.
    pub fn client(&self) -> reqwest::Result<reqwest::Client> {
        reqwest::Client::builder()
            .user_agent("osiris")
            .connect_timeout(self.connect_timeout())
            .timeout(self.timeout())
            .build()
    }
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self {
            connect_timeout_seconds: 5,
            timeout_seconds: 20,
        }
    }
}
