//! Records how a task run ended. If recording fails, the failure is logged but the task is not run
//! again, because its work has already been done (or not) either way.

use crate::config::settings::tasks::TasksSettings;
use crate::use_cases::lib::{database::Db, task_outcomes::Recorded};
use osiris_domain::TaskOutcome;
use osiris_tasks::Task;

pub async fn record_outcome(
    db: &Db,
    settings: &TasksSettings,
    task: &Task,
    outcome: TaskOutcome,
    detail: Option<&str>,
) {
    tracing::info!(task = %task.name, kind = task.payload.kind(), outcome = outcome.as_str(), detail, "task ran");
    let recorded = Recorded {
        task_name: &task.name,
        kind: task.payload.kind(),
        connected_account_id: task.payload.connected_account_id(),
        outcome,
        detail,
    };
    if let Err(error) = db
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
