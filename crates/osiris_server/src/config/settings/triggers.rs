//! `[triggers]`: how often a trigger that checks on a schedule (a watched item) checks, and how
//! large a delivery osiris reads.

use super::seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TriggersSettings {
    /// Between two scheduled checks of one trigger.
    pub check_seconds: u64,
    /// The largest delivery body read at `/webhooks/triggers/{id}`; a larger one is answered 413.
    pub max_delivery_bytes: u64,
}

impl TriggersSettings {
    pub fn check_interval(&self) -> time::Duration {
        seconds(self.check_seconds)
    }

    pub fn max_delivery(&self) -> usize {
        usize::try_from(self.max_delivery_bytes).unwrap_or(usize::MAX)
    }

    /// Refuses a check interval the task scheduler can't schedule that far ahead.
    pub fn check_against(&self, lead_limit: Option<std::time::Duration>) -> anyhow::Result<()> {
        if let Some(limit) = lead_limit {
            anyhow::ensure!(
                std::time::Duration::from_secs(self.check_seconds) < limit,
                "triggers.check_seconds must be under the task scheduler's limit of {} seconds",
                limit.as_secs()
            );
        }
        Ok(())
    }
}

impl Default for TriggersSettings {
    fn default() -> Self {
        Self {
            // OpenConnector's pull request watch checks every five minutes.
            check_seconds: 300,
            // 25 MiB: the largest webhook payload the providers osiris implements send.
            max_delivery_bytes: 26_214_400,
        }
    }
}
