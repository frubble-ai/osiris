//! `[tasks]`: how long task outcomes are kept, and how the local scheduler retries.

use super::seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TasksSettings {
    /// How long the outcome of a task run is kept.
    pub outcome_retention_seconds: u64,
    /// The most expired outcomes deleted each time a new one is recorded.
    pub outcome_prune_batch: i64,
    /// The most tasks the worker runs at once; more wait. Keep it under
    /// `database.max_connections`, since each task may hold a connection.
    pub max_concurrent: usize,
    /// How long scheduling one task may take before it counts as failed.
    pub schedule_timeout_seconds: u64,
    /// How long Google's OIDC signing keys are cached before they are fetched again.
    pub oidc_keys_ttl_seconds: u64,
    /// The shortest time between two fetches of those keys for a key id not yet in the cache.
    pub oidc_keys_refetch_seconds: u64,
    pub local: LocalTasksSettings,
}

/// `[tasks.local]`: the scheduler used in local development (`OSIRIS_TASKS=local`).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct LocalTasksSettings {
    /// How long to wait before sending a task again after the worker refused it.
    pub retry_delay_seconds: u64,
    /// How many times a task is sent before the scheduler gives up on it.
    pub max_attempts: u32,
}

impl TasksSettings {
    pub fn schedule_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.schedule_timeout_seconds)
    }

    pub fn outcome_retention(&self) -> time::Duration {
        seconds(self.outcome_retention_seconds)
    }
}

impl Default for TasksSettings {
    fn default() -> Self {
        Self {
            outcome_retention_seconds: 2_592_000,
            outcome_prune_batch: 100,
            max_concurrent: 4,
            schedule_timeout_seconds: 10,
            oidc_keys_ttl_seconds: 3600,
            oidc_keys_refetch_seconds: 30,
            local: LocalTasksSettings::default(),
        }
    }
}

impl Default for LocalTasksSettings {
    fn default() -> Self {
        Self {
            retry_delay_seconds: 5,
            max_attempts: 5,
        }
    }
}
