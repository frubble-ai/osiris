//! A GitHub Actions workflow, as GitHub answers it.

use crate::github::types::GithubWorkflow;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkflowAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
}

impl From<WorkflowAnswer> for GithubWorkflow {
    /// A GitHub Actions workflow, in our terms.
    fn from(answer: WorkflowAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            path: answer.path,
            state: answer.state,
            url: answer.url,
        }
    }
}
