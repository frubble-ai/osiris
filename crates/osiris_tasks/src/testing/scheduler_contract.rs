//! What every task scheduler owes the use cases, checked the same way for each one: a name is
//! scheduled once, scheduling it again (even after it was cancelled) creates nothing new, and
//! cancelling a task that doesn't exist is not an error. A harness makes a scheduler that talks to a
//! service (Cloud Tasks) answer each step; one that doesn't (memory, local) needs no arranging.

use super::generate_task::generate_task;
use crate::{Scheduled, TaskScheduler};
use async_trait::async_trait;

#[async_trait]
pub trait SchedulerHarness: Send + Sync {
    fn scheduler(&self) -> &dyn TaskScheduler;

    /// The service creates the task `name`.
    async fn answer_created(&self, _name: &str) {}

    /// The service already has (or recently had) a task `name`.
    async fn answer_already_exists(&self, _name: &str) {}

    /// The service deletes the task `name`.
    async fn answer_cancelled(&self, _name: &str) {}

    /// The service has no task `name` to delete.
    async fn answer_cancel_gone(&self, _name: &str) {}
}

/// Runs every step of the contract against `harness`'s scheduler.
pub async fn task_scheduler_contract(harness: &dyn SchedulerHarness) {
    schedules_a_new_task_once(harness).await;
    never_reuses_a_cancelled_name(harness).await;
    cancels_a_task_that_does_not_exist_without_an_error(harness).await;
}

async fn schedules_a_new_task_once(harness: &dyn SchedulerHarness) {
    let task = generate_task();
    harness.answer_created(&task.name).await;
    assert_eq!(
        harness
            .scheduler()
            .schedule(&task)
            .await
            .expect("scheduled"),
        Scheduled::Created
    );
    harness.answer_already_exists(&task.name).await;
    assert_eq!(
        harness
            .scheduler()
            .schedule(&task)
            .await
            .expect("scheduled"),
        Scheduled::AlreadyScheduled,
        "the same name twice makes one task"
    );
}

async fn never_reuses_a_cancelled_name(harness: &dyn SchedulerHarness) {
    let task = generate_task();
    harness.answer_created(&task.name).await;
    harness
        .scheduler()
        .schedule(&task)
        .await
        .expect("scheduled");
    harness.answer_cancelled(&task.name).await;
    harness
        .scheduler()
        .cancel(&task.name)
        .await
        .expect("cancelled");
    harness.answer_already_exists(&task.name).await;
    assert_eq!(
        harness
            .scheduler()
            .schedule(&task)
            .await
            .expect("scheduled"),
        Scheduled::AlreadyScheduled,
        "a cancelled task's name is not used again"
    );
}

async fn cancels_a_task_that_does_not_exist_without_an_error(harness: &dyn SchedulerHarness) {
    let task = generate_task();
    harness.answer_cancel_gone(&task.name).await;
    harness
        .scheduler()
        .cancel(&task.name)
        .await
        .expect("cancelling a task that is gone is fine");
}
