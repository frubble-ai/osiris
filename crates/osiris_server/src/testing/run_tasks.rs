//! Running scheduled tasks through the use case the worker calls, as if the scheduler had sent
//! them.

use super::TestContext;
use crate::use_cases::{UseCaseResult, cases::run_task::RunTaskArgs};
use osiris_domain::TaskOutcome;
use osiris_tasks::Task;

impl TestContext {
    /// Runs `task`; `Ok` is what the worker acknowledges, an error what it asks to retry.
    pub async fn run(&self, task: &Task) -> UseCaseResult<TaskOutcome> {
        self.use_cases.run_task(RunTaskArgs { task }).await
    }

    /// Takes the pending task `name` out of the scheduler and runs it.
    pub async fn run_pending(&self, name: &str) -> UseCaseResult<TaskOutcome> {
        let task = self.take_pending(name);
        self.run(&task).await
    }
}
