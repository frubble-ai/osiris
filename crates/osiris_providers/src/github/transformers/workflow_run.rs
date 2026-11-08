//! A run of a workflow, as GitHub answers it.

use crate::github::types::GithubWorkflowRun;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkflowRunAnswer {
    pub id: u64,
    pub name: Option<String>,
    #[serde(default)]
    pub display_title: String,
    pub workflow_id: u64,
    #[serde(default)]
    pub event: String,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub head_branch: Option<String>,
    #[serde(default)]
    pub head_sha: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub run_number: i64,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

impl From<WorkflowRunAnswer> for GithubWorkflowRun {
    /// A run of a workflow, in our terms.
    fn from(answer: WorkflowRunAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            display_title: answer.display_title,
            workflow_id: answer.workflow_id.to_string(),
            event: answer.event,
            status: answer.status,
            conclusion: answer.conclusion,
            head_branch: answer.head_branch,
            head_sha: answer.head_sha,
            url: answer.url,
            run_number: answer.run_number,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
        }
    }
}
