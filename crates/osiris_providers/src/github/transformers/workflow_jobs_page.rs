//! A page of a workflow run's jobs, as GitHub wraps it.

use super::workflow_job::WorkflowJobAnswer;
use serde::Deserialize;

/// A page of a workflow run's jobs, as GitHub wraps it.
#[derive(Deserialize)]
pub struct WorkflowJobsPage {
    #[serde(default)]
    pub jobs: Vec<WorkflowJobAnswer>,
}
