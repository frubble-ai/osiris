//! The trait every task scheduler implements. The core depends only on this trait.

use crate::{Scheduled, Task, TaskError};
use async_trait::async_trait;

#[async_trait]
pub trait TaskScheduler: Send + Sync {
    /// Schedules `task` to be sent to the worker at its `run_at` time. If a task with the same
    /// name already exists or ran recently, nothing new is created.
    async fn schedule(&self, task: &Task) -> Result<Scheduled, TaskError>;

    /// How far ahead this scheduler accepts a task's `run_at`, if it has a limit.
    fn max_lead_time(&self) -> Option<std::time::Duration> {
        None
    }

    /// Deletes the pending task called `name`. Deleting a task that no longer exists is not an
    /// error.
    async fn cancel(&self, name: &str) -> Result<(), TaskError>;
}
