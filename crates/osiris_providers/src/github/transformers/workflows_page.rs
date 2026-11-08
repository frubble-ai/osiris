//! A page of a repository's workflows, as GitHub wraps it.

use super::workflow::WorkflowAnswer;
use serde::Deserialize;

/// A page of a repository's workflows, as GitHub wraps it.
#[derive(Deserialize)]
pub struct WorkflowsPage {
    #[serde(default)]
    pub workflows: Vec<WorkflowAnswer>,
}
