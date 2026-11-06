//! Runs one task that the scheduler sent to the worker, and records how it ended. Returning `Ok`
//! tells the scheduler the task is done. Returning an error makes the scheduler run the same task
//! again later.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        expire_consent::ExpireConsentArgs, publish_event_retry::PublishEventRetryArgs,
        run_refresh_task::RunRefreshTaskArgs,
    },
    lib::tasks::record_outcome::record_outcome,
};
use osiris_domain::TaskOutcome;
use osiris_tasks::{Task, TaskPayload};

pub struct RunTaskArgs<'a> {
    pub task: &'a Task,
}

impl UseCases {
    pub async fn run_task(&self, args: RunTaskArgs<'_>) -> UseCaseResult<TaskOutcome> {
        self.run("run_task", async {
            let ctx = self.context();
            let task = args.task;
            let ran = match &task.payload {
                TaskPayload::ConnectionRefresh(refresh) => {
                    self.run_refresh_task(RunRefreshTaskArgs {
                        name: &task.name,
                        task: refresh,
                    })
                    .await
                }
                TaskPayload::ConsentExpire(consent) => {
                    self.expire_consent(ExpireConsentArgs { task: consent }).await
                }
                TaskPayload::EventPublish(event) => {
                    self.publish_event_retry(PublishEventRetryArgs { task: event })
                        .await
                }
            };
            let settings = &ctx.config.settings.tasks;
            match ran {
                Ok((outcome, detail)) => {
                    record_outcome(&ctx.db, settings, task, outcome, detail).await;
                    Ok(outcome)
                }
                Err(report) => {
                    tracing::warn!(error = ?report, task = %task.name, "task failed; the scheduler will run it again");
                    // The recorded detail is a fixed message; the full error stays in the logs.
                    record_outcome(
                        &ctx.db,
                        settings,
                        task,
                        TaskOutcome::Failed,
                        Some("failed for now; will run again"),
                    )
                    .await;
                    Err(report)
                }
            }
        })
        .await
    }
}
