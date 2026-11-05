//! How a task run ended. One outcome is recorded for every run.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskOutcome {
    /// The connection's tokens were refreshed and the next refresh was scheduled.
    Refreshed,
    /// The connection was disabled because its last refresh could not schedule the next one. Its
    /// refresh is now scheduled and it is active again.
    Rescheduled,
    /// The task was out of date, so it did nothing. For example, the connection was deleted, is no
    /// longer active, has newer tokens, or its consent was already finished.
    SkippedStale,
    /// The task failed. A temporary failure is tried again later. When the provider refuses a
    /// refresh permanently, the connection needs reconnecting and the task is not tried again.
    Failed,
    /// The consent was still open when its time ran out, and is now marked as failed.
    ConsentExpired,
    /// The event was published.
    Published,
}

impl TaskOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Refreshed => "refreshed",
            Self::Rescheduled => "rescheduled",
            Self::SkippedStale => "skipped_stale",
            Self::Failed => "failed",
            Self::ConsentExpired => "consent_expired",
            Self::Published => "published",
        }
    }
}
