//! Schedules one task. Whether the scheduler creates it or already has a task with that name, the
//! job ends up scheduled exactly once.

use crate::core::{Core, CoreResult, errors::internal};
use osiris_tasks::Task;

impl Core {
    pub(crate) async fn schedule(&self, task: &Task) -> CoreResult<()> {
        let scheduled = self.tasks.schedule(task).await.map_err(internal)?;
        tracing::debug!(task = %task.name, kind = task.payload.kind(), run_at = %task.run_at, ?scheduled, "task scheduled");
        Ok(())
    }
}
