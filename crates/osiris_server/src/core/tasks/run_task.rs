//! Runs one task that the scheduler sent to the worker, and records how it ended. Returning `Ok`
//! tells the scheduler the task is done. Returning an error makes the scheduler run the same task
//! again later.

use crate::core::{Core, CoreResult};
use osiris_domain::TaskOutcome;
use osiris_tasks::{Task, TaskPayload};

impl Core {
    pub async fn run_task(&self, task: &Task) -> CoreResult<TaskOutcome> {
        let ran = match &task.payload {
            TaskPayload::ConnectionRefresh(refresh) => self.run_refresh(&task.name, refresh).await,
            TaskPayload::ConsentExpire(consent) => self.expire_consent(consent).await,
            TaskPayload::EventPublish(event) => self.publish_event_task(event).await,
        };
        match ran {
            Ok((outcome, detail)) => {
                self.record(task, outcome, detail).await;
                Ok(outcome)
            }
            Err(report) => {
                tracing::warn!(error = ?report, task = %task.name, "task failed; the scheduler will run it again");
                // The recorded detail is a fixed message; the full error stays in the logs.
                self.record(
                    task,
                    TaskOutcome::Failed,
                    Some("failed for now; will run again"),
                )
                .await;
                Err(report)
            }
        }
    }
}
