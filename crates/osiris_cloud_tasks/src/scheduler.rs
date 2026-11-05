//! The `TaskScheduler` implementation for Cloud Tasks.

use crate::CloudTasksConfig;
use async_trait::async_trait;
use google_cloud_gax::error::rpc::Code;
use google_cloud_tasks_v2::{
    client::CloudTasks,
    model::{HttpMethod, HttpRequest, OidcToken, Task as CloudTask},
};
use osiris_tasks::{Scheduled, Task, TaskError, TaskScheduler};

/// Cloud Tasks schedules a task at most 30 days ahead.
pub const MAX_LEAD_TIME: std::time::Duration = std::time::Duration::from_hours(30 * 24);

pub struct CloudTasksScheduler {
    client: CloudTasks,
    config: CloudTasksConfig,
}

impl CloudTasksScheduler {
    pub async fn connect(config: CloudTasksConfig) -> Result<Self, TaskError> {
        let client = CloudTasks::builder()
            .build()
            .await
            .map_err(|error| TaskError::new(error.to_string()))?;
        Ok(Self { client, config })
    }

    /// A scheduler over an already built client (a test points one at a fake).
    pub fn with_client(client: CloudTasks, config: CloudTasksConfig) -> Self {
        Self { client, config }
    }

    fn task_path(&self, name: &str) -> String {
        format!("{}/tasks/{name}", self.config.queue_path())
    }
}

#[async_trait]
impl TaskScheduler for CloudTasksScheduler {
    async fn schedule(&self, task: &Task) -> Result<Scheduled, TaskError> {
        let body = serde_json::to_vec(task).map_err(|error| TaskError::new(error.to_string()))?;
        let run_at = google_cloud_wkt::Timestamp::try_from(task.run_at)
            .map_err(|error| TaskError::new(error.to_string()))?;
        let request = HttpRequest::new()
            .set_url(self.config.handler_url.clone())
            .set_http_method(HttpMethod::Post)
            .set_headers([("Content-Type", "application/json")])
            .set_body(body)
            .set_oidc_token(
                OidcToken::new()
                    .set_service_account_email(self.config.service_account_email.clone())
                    .set_audience(self.config.audience.clone()),
            );
        let created = self
            .client
            .create_task()
            .set_parent(self.config.queue_path())
            .set_task(
                CloudTask::new()
                    .set_name(self.task_path(&task.name))
                    .set_schedule_time(run_at)
                    .set_http_request(request),
            )
            .send()
            .await;
        match created {
            Ok(_) => Ok(Scheduled::Created),
            Err(error)
                if error
                    .status()
                    .is_some_and(|status| status.code == Code::AlreadyExists) =>
            {
                Ok(Scheduled::AlreadyScheduled)
            }
            Err(error) => Err(TaskError::new(error.to_string())),
        }
    }

    fn max_lead_time(&self) -> Option<std::time::Duration> {
        Some(MAX_LEAD_TIME)
    }

    async fn cancel(&self, name: &str) -> Result<(), TaskError> {
        match self
            .client
            .delete_task()
            .set_name(self.task_path(name))
            .send()
            .await
        {
            Ok(()) => Ok(()),
            Err(error)
                if error
                    .status()
                    .is_some_and(|status| status.code == Code::NotFound) =>
            {
                Ok(())
            }
            Err(error) => Err(TaskError::new(error.to_string())),
        }
    }
}
