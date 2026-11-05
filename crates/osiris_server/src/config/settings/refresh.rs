//! `[refresh]`: when a connection's refresh task runs. When a refresh fails, the task scheduler's
//! own retry policy decides when to try again, not osiris.

use super::seconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
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
    /// The keepalive must be shorter than how far ahead the task scheduler can schedule, or every
    /// keepalive refresh would fail to schedule and leave its connection disabled.
    pub fn check_against(&self, lead_limit: Option<std::time::Duration>) -> anyhow::Result<()> {
        if let Some(limit) = lead_limit {
            anyhow::ensure!(
                std::time::Duration::from_secs(self.keepalive_seconds) < limit,
                "refresh.keepalive_seconds must be under the task scheduler's limit of {} seconds",
                limit.as_secs()
            );
        }
        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    const THIRTY_DAYS: Duration = Duration::from_hours(30 * 24);

    #[test]
    fn accepts_a_keepalive_under_the_schedulers_limit_or_a_scheduler_without_one() {
        let settings = RefreshSettings::default();
        assert!(settings.check_against(Some(THIRTY_DAYS)).is_ok());
        assert!(settings.check_against(None).is_ok());
    }

    #[test]
    fn refuses_a_keepalive_the_scheduler_could_not_schedule() {
        for seconds in [THIRTY_DAYS.as_secs(), THIRTY_DAYS.as_secs() + 1] {
            let settings = RefreshSettings {
                keepalive_seconds: seconds,
                ..RefreshSettings::default()
            };
            let error = settings.check_against(Some(THIRTY_DAYS)).unwrap_err();
            assert!(
                error.to_string().contains("refresh.keepalive_seconds"),
                "{seconds}"
            );
        }
    }
}
