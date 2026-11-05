//! Which task scheduler runs osiris's scheduled tasks, set by `OSIRIS_TASKS`: `cloud_tasks` when
//! deployed, `local` in development, and each one's own configuration. `build_use_case_context`
//! builds the chosen one; the use cases only see the `TaskScheduler` trait.

use crate::config::env_vars::{optional, required, secret};
use anyhow::bail;
use osiris_cloud_tasks::CloudTasksConfig;

#[derive(Debug, Clone)]
pub enum TasksConfig {
    CloudTasks(CloudTasksConfig),
    /// Timers inside the process, which send each task to the worker when it is due, with the
    /// worker token (`OSIRIS_WORKER_TOKEN`, at least 32 characters) the worker requires.
    Local {
        handler_url: String,
        token: osiris_common::secret::Secret,
    },
}

impl TasksConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let worker_url = required("OSIRIS_WORKER_URL")?
            .trim_end_matches('/')
            .to_owned();
        let handler_url = format!("{worker_url}/tasks");
        match required("OSIRIS_TASKS")?.as_str() {
            "cloud_tasks" => Ok(Self::CloudTasks(CloudTasksConfig {
                project: required("CLOUD_TASKS_PROJECT")?,
                location: required("CLOUD_TASKS_LOCATION")?,
                queue: required("CLOUD_TASKS_QUEUE")?,
                handler_url,
                service_account_email: required("CLOUD_TASKS_SERVICE_ACCOUNT")?,
                audience: optional("CLOUD_TASKS_AUDIENCE").unwrap_or(worker_url),
            })),
            "local" => Ok(Self::Local {
                handler_url,
                token: secret("OSIRIS_WORKER_TOKEN")?,
            }),
            other => bail!("OSIRIS_TASKS must be cloud_tasks or local, not {other:?}"),
        }
    }
}
