//! The schedulers that need no outside service, held to the contract every scheduler is.

use super::{
    local::{LocalConfig, LocalScheduler},
    memory::MemoryScheduler,
};
use crate::{
    TaskScheduler,
    testing::{SchedulerHarness, task_scheduler_contract},
};
use std::time::Duration;

struct Plain<S>(S);

impl<S: TaskScheduler> SchedulerHarness for Plain<S> {
    fn scheduler(&self) -> &dyn TaskScheduler {
        &self.0
    }
}

#[tokio::test]
async fn the_memory_scheduler_keeps_the_contract() {
    task_scheduler_contract(&Plain(MemoryScheduler::default())).await;
}

#[tokio::test]
async fn the_local_scheduler_keeps_the_contract() {
    // The tasks are due a day from now, so none is sent to the (absent) worker.
    let scheduler = LocalScheduler::new(LocalConfig {
        handler_url: "http://worker.invalid/tasks".into(),
        token: None,
        retry_delay: Duration::from_secs(1),
        max_attempts: 1,
    });
    task_scheduler_contract(&Plain(scheduler)).await;
}
