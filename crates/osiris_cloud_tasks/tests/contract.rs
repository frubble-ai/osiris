//! The Cloud Tasks scheduler held to the contract every scheduler is, against Cloud Tasks' REST API
//! faked at the network: the real client runs, only the HTTP answers are the test's.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use async_trait::async_trait;
use google_cloud_tasks_v2::client::CloudTasks;
use osiris_cloud_tasks::{CloudTasksConfig, CloudTasksScheduler};
use osiris_tasks::{
    TaskScheduler,
    testing::{SchedulerHarness, task_scheduler_contract},
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, method, path},
};

fn config() -> CloudTasksConfig {
    CloudTasksConfig {
        project: "osiris-test".into(),
        location: "europe-west1".into(),
        queue: "osiris".into(),
        handler_url: "https://worker.test/tasks".into(),
        service_account_email: "tasks@osiris-test.iam.gserviceaccount.com".into(),
        audience: "https://worker.test".into(),
    }
}

struct FakeCloudTasks {
    server: MockServer,
    scheduler: CloudTasksScheduler,
}

impl FakeCloudTasks {
    async fn start() -> Self {
        let server = MockServer::start().await;
        let client = CloudTasks::builder()
            .with_endpoint(server.uri())
            .with_credentials(google_cloud_auth::credentials::anonymous::Builder::new().build())
            .build()
            .await
            .unwrap();
        Self {
            scheduler: CloudTasksScheduler::with_client(client, config()),
            server,
        }
    }

    fn tasks_path() -> String {
        format!("/v2/{}/tasks", config().queue_path())
    }

    /// One answer to the next create of `name`.
    async fn answer_create(&self, name: &str, status: u16, body: serde_json::Value) {
        Mock::given(method("POST"))
            .and(path(Self::tasks_path()))
            .and(body_string_contains(name))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .up_to_n_times(1)
            .mount(&self.server)
            .await;
    }

    /// One answer to the next delete of `name`.
    async fn answer_delete(&self, name: &str, status: u16, body: serde_json::Value) {
        Mock::given(method("DELETE"))
            .and(path(format!("{}/{name}", Self::tasks_path())))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .up_to_n_times(1)
            .mount(&self.server)
            .await;
    }
}

#[async_trait]
impl SchedulerHarness for FakeCloudTasks {
    fn scheduler(&self) -> &dyn TaskScheduler {
        &self.scheduler
    }

    async fn answer_created(&self, name: &str) {
        let full = format!("{}/{name}", Self::tasks_path().trim_start_matches("/v2/"));
        self.answer_create(name, 200, json!({ "name": full })).await;
    }

    async fn answer_already_exists(&self, name: &str) {
        self.answer_create(
            name,
            409,
            json!({ "error": { "code": 409, "message": "Requested entity already exists", "status": "ALREADY_EXISTS" } }),
        )
        .await;
    }

    async fn answer_cancelled(&self, name: &str) {
        self.answer_delete(name, 200, json!({})).await;
    }

    async fn answer_cancel_gone(&self, name: &str) {
        self.answer_delete(
            name,
            404,
            json!({ "error": { "code": 404, "message": "Requested entity was not found", "status": "NOT_FOUND" } }),
        )
        .await;
    }
}

#[tokio::test]
async fn the_cloud_tasks_scheduler_keeps_the_contract() {
    task_scheduler_contract(&FakeCloudTasks::start().await).await;
}
