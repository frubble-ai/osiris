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
    #[serde(default)]
    pub badge_url: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
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
            badge_url: answer.badge_url,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
        }
    }
}
