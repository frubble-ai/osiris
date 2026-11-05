//! Runs a `consent.expire` task: marks the consent as failed if it is still open. If the consent
//! was already finished, the task is out of date and does nothing.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::TaskOutcome;
use osiris_tasks::ConsentTask;

impl Core {
    pub(crate) async fn expire_consent(
        &self,
        task: &ConsentTask,
    ) -> CoreResult<(TaskOutcome, Option<&'static str>)> {
        Ok(
            if self
                .store
                .fail_open_consent(&task.connected_account_id)
                .await
                .switch()?
            {
                (TaskOutcome::ConsentExpired, None)
            } else {
                (TaskOutcome::SkippedStale, None)
            },
        )
    }
}
