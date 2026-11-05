//! `[database]`: how many connections each process keeps. The Cloud SQL instance is shared with
//! other services, so the default is low; raise it only with the instance's connection limit in
//! mind.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct DatabaseSettings {
    /// The most connections one process opens.
    pub max_connections: u32,
    /// How long to wait for a connection, at startup and afterwards, before giving up.
    pub connect_timeout_seconds: u64,
    /// How long a refresh waits for another refresh's row lock before giving up.
    pub lock_timeout_seconds: u64,
}

impl DatabaseSettings {
    pub fn connect_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.connect_timeout_seconds)
    }

    pub fn lock_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.lock_timeout_seconds)
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            max_connections: 5,
            connect_timeout_seconds: 10,
            lock_timeout_seconds: 10,
        }
    }
}
