//! Records how a task run ended. If recording fails, the failure is logged but the task is not run
//! again, because its work has already been done (or not) either way.

use crate::core::Core;
use osiris_domain::TaskOutcome;
use osiris_storage::task_outcomes::Recorded;
use osiris_tasks::Task;

impl Core {
    pub(crate) async fn record(&self, task: &Task, outcome: TaskOutcome, detail: Option<&str>) {
        tracing::info!(task = %task.name, kind = task.payload.kind(), outcome = outcome.as_str(), detail, "task ran");
        let settings = &self.settings.tasks;
        let recorded = Recorded {
            task_name: &task.name,
            kind: task.payload.kind(),
            connected_account_id: task.payload.connected_account_id(),
            outcome,
            detail,
        };
        if let Err(error) = self
            .store
            .record_task_outcome(
                recorded,
                settings.outcome_retention(),
                settings.outcome_prune_batch,
            )
            .await
        {
            tracing::error!(?error, task = %task.name, "recording the task's outcome failed");
        }
    }
}
