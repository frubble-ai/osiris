//! Runs a `connection.refresh` task. The connection is refreshed only if it is still active and
//! still on the token generation the task was created for. Otherwise the task is out of date and
//! does nothing.

use crate::core::{
    Core, CoreResult,
    refresh::{Refresh, RunningTask, When},
};
use osiris_domain::TaskOutcome;
use osiris_tasks::RefreshTask;

impl Core {
    pub(crate) async fn run_refresh(
        &self,
        name: &str,
        task: &RefreshTask,
    ) -> CoreResult<(TaskOutcome, Option<&'static str>)> {
        let running = RunningTask {
            name,
            generation: task.generation,
        };
        Ok(
            match self
                .refresh(&task.connected_account_id, When::Now, Some(running))
                .await?
            {
                Refresh::Refreshed(_) => (TaskOutcome::Refreshed, None),
                Refresh::Restored(_) => (TaskOutcome::Rescheduled, None),
                Refresh::Stale | Refresh::Unchanged(_) => (TaskOutcome::SkippedStale, None),
                Refresh::Ended(_) => (
                    TaskOutcome::Failed,
                    Some("the provider permanently refused the refresh; the user must reconnect"),
                ),
            },
        )
    }
}
