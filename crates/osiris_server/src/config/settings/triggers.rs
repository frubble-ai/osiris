//! `[triggers]`: how often a trigger that checks on a schedule (a watched pull request) checks.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TriggersSettings {
    /// Between two scheduled checks of one trigger.
    pub check_seconds: u64,
}

impl TriggersSettings {
    pub fn check_interval(&self) -> time::Duration {
        time::Duration::seconds(i64::try_from(self.check_seconds).unwrap_or(i64::MAX))
    }
}

impl Default for TriggersSettings {
    fn default() -> Self {
        // OpenConnector's pull request watch checks every five minutes.
        Self { check_seconds: 300 }
    }
}
