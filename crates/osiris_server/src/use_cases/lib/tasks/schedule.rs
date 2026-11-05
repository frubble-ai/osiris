//! Schedules one task. Whether the scheduler creates it or already has a task with that name, the
//! job ends up scheduled exactly once. A scheduler that doesn't answer within the timeout counts
//! as failed.

use super::Scheduling;
use crate::use_cases::lib::shared::domain_errors::{UseCaseResult, internal};
use osiris_tasks::{Task, TaskError};

pub async fn schedule(scheduling: Scheduling<'_>, task: &Task) -> UseCaseResult<()> {
    let scheduled = tokio::time::timeout(scheduling.timeout, scheduling.tasks.schedule(task))
        .await
        .unwrap_or_else(|_| Err(TaskError::new("the task scheduler did not answer in time")))
        .map_err(internal)?;
    tracing::debug!(task = %task.name, kind = task.payload.kind(), run_at = %task.run_at, ?scheduled, "task scheduled");
    Ok(())
}
