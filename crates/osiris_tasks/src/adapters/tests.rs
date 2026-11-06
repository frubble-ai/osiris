//! The schedulers that need no outside service, held to the contract every scheduler is, and the
//! local scheduler's delivery to the worker.

use super::{
    local::{LocalConfig, LocalScheduler},
    memory::MemoryScheduler,
    mocks::{mock_scheduler_harness::MockSchedulerHarness, mock_worker::MockWorker},
};
use crate::TaskScheduler;
use crate::testing::{generate_task::generate_task, task_scheduler_contract};
use fake::{Fake, faker::internet::en::Password};
use osiris_common::secret::secret;
use std::time::Duration;
use time::OffsetDateTime;
use wiremock::MockServer;

const LONG_ENOUGH: Duration = Duration::from_secs(10);
const RETRY_DELAY: Duration = Duration::from_millis(20);

#[tokio::test]
async fn the_memory_scheduler_keeps_the_contract() {
    task_scheduler_contract(&MockSchedulerHarness(MemoryScheduler::default())).await;
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
    task_scheduler_contract(&MockSchedulerHarness(scheduler)).await;
}

#[tokio::test]
async fn the_local_scheduler_sends_a_due_task_with_the_token_and_sends_it_again_after_a_refusal() {
    let server = MockServer::start().await;
    let worker = MockWorker::on(&server);
    worker.answer_once(500).await;
    worker.answer_once(204).await;
    let token: String = Password(32..40).fake();
    let scheduler = LocalScheduler::new(LocalConfig {
        handler_url: worker.url(),
        token: Some(secret(&token)),
        retry_delay: RETRY_DELAY,
        max_attempts: 3,
    });
    let mut task = generate_task();
    task.run_at = OffsetDateTime::now_utc();

    scheduler.schedule(&task).await.unwrap();
    tokio::time::timeout(LONG_ENOUGH, async {
        while worker.received().await.len() < 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the task is sent again after the refusal");

    // A scheduler that kept retrying after the 204 would send again within a retry delay.
    tokio::time::sleep(RETRY_DELAY * 5).await;
    let sent = worker.received().await;
    assert_eq!(sent.len(), 2, "it stops once the worker answers 2xx");
    for (authorization, body) in sent {
        assert_eq!(authorization, Some(format!("Bearer {token}")));
        assert_eq!(body["name"], task.name.as_str());
    }
}
