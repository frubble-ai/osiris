//! The queue's location, the worker URL its tasks are sent to, and the identity they are sent as.

#[derive(Debug, Clone)]
pub struct CloudTasksConfig {
    pub project: String,
    pub location: String,
    pub queue: String,
    /// The worker's task endpoint, e.g. `https://osiris-worker-….run.app/tasks`.
    pub handler_url: String,
    /// The service account the OIDC token is issued for. It must be allowed to invoke the worker.
    pub service_account_email: String,
    /// The audience of the OIDC token. Cloud Run expects the worker's URL.
    pub audience: String,
}

impl CloudTasksConfig {
    pub fn queue_path(&self) -> String {
        format!(
            "projects/{}/locations/{}/queues/{}",
            self.project, self.location, self.queue
        )
    }
}
