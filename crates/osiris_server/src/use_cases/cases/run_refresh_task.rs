//! Runs a `connection.refresh` task. The connection is refreshed only if it is still active and
//! still on the token generation the task was created for. Otherwise the task is out of date and
//! does nothing.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::refresh_connected_account::RefreshConnectedAccountArgs,
    lib::refresh::{Refresh, RunningTask, When},
};
use osiris_domain::TaskOutcome;
use osiris_tasks::RefreshTask;

pub struct RunRefreshTaskArgs<'a> {
    pub name: &'a str,
    pub task: &'a RefreshTask,
}

impl UseCases {
    pub async fn run_refresh_task(
        &self,
        args: RunRefreshTaskArgs<'_>,
    ) -> UseCaseResult<(TaskOutcome, Option<&'static str>)> {
        self.run("run_refresh_task", async {
            let running = RunningTask {
                name: args.name,
                generation: args.task.generation,
            };
            Ok(
                match self
                    .refresh_connected_account(RefreshConnectedAccountArgs {
                        id: &args.task.connected_account_id,
                        when: When::Now,
                        running: Some(running),
                    })
                    .await?
                {
                    Refresh::Refreshed(_) => (TaskOutcome::Refreshed, None),
                    Refresh::Restored(_) => (TaskOutcome::Rescheduled, None),
                    Refresh::Stale | Refresh::Unchanged(_) => (TaskOutcome::SkippedStale, None),
                    Refresh::Ended(_) => (
                        TaskOutcome::Failed,
                        Some(
                            "the provider permanently refused the refresh; the user must reconnect",
                        ),
                    ),
                },
            )
        })
        .await
    }
}
