//! Schedules one task. Whether the scheduler creates it or already has a task with that name, the
//! job ends up scheduled exactly once. A scheduler that doesn't answer within
//! `[tasks] schedule_timeout` counts as failed.

use crate::core::{Core, CoreResult, errors::internal};
use osiris_tasks::{Task, TaskError};

impl Core {
    pub(crate) async fn schedule(&self, task: &Task) -> CoreResult<()> {
        let scheduled = tokio::time::timeout(
            self.settings.tasks.schedule_timeout(),
            self.tasks.schedule(task),
        )
        .await
        .unwrap_or_else(|_| Err(TaskError::new("the task scheduler did not answer in time")))
        .map_err(internal)?;
        tracing::debug!(task = %task.name, kind = task.payload.kind(), run_at = %task.run_at, ?scheduled, "task scheduled");
        Ok(())
    }
}
