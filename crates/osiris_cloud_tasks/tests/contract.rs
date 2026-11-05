//! The Cloud Tasks scheduler held to the contract every scheduler is, against Cloud Tasks' REST API
//! faked at the network: the real client runs, only the HTTP answers are the test's.

use osiris_cloud_tasks::mocks::mock_cloud_tasks::MockCloudTasks;
use osiris_tasks::testing::task_scheduler_contract;

#[tokio::test]
async fn the_cloud_tasks_scheduler_keeps_the_contract() {
    task_scheduler_contract(&MockCloudTasks::start().await).await;
}
