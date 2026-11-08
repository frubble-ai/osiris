//! A page of a repository's workflow runs, as GitHub wraps it.

use super::workflow_run::WorkflowRunAnswer;
use serde::Deserialize;

/// A page of a repository's workflow runs, as GitHub wraps it.
#[derive(Deserialize)]
pub struct WorkflowRunsPage {
    #[serde(default)]
    pub workflow_runs: Vec<WorkflowRunAnswer>,
}
