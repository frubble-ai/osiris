//! Runs a `consent.expire` task: marks the consent as failed if it is still open. If the consent
//! was already finished, the task is out of date and does nothing.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::TaskOutcome;
use osiris_tasks::ConsentTask;

pub struct ExpireConsentArgs<'a> {
    pub task: &'a ConsentTask,
}

impl UseCases {
    pub async fn expire_consent(
        &self,
        args: ExpireConsentArgs<'_>,
    ) -> UseCaseResult<(TaskOutcome, Option<&'static str>)> {
        self.run("expire_consent", async {
            Ok(
                if self
                    .context()
                    .db
                    .fail_open_consent(&args.task.connected_account_id)
                    .await?
                {
                    (TaskOutcome::ConsentExpired, None)
                } else {
                    (TaskOutcome::SkippedStale, None)
                },
            )
        })
        .await
    }
}
