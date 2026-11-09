//! A job of a workflow run, as GitHub answers it.

use crate::github::types::{GithubWorkflowJob, GithubWorkflowStep};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkflowStepAnswer {
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    pub conclusion: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

impl From<WorkflowStepAnswer> for GithubWorkflowStep {
    /// A step of a job, in our terms.
    fn from(answer: WorkflowStepAnswer) -> Self {
        Self {
            number: answer.number,
            name: answer.name,
            status: answer.status,
            conclusion: answer.conclusion,
            started_at: answer.started_at,
            completed_at: answer.completed_at,
        }
    }
}

#[derive(Deserialize)]
pub struct WorkflowJobAnswer {
    pub id: u64,
    pub run_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    pub conclusion: Option<String>,
    #[serde(rename = "html_url")]
    pub url: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    #[serde(default)]
    pub steps: Vec<WorkflowStepAnswer>,
    #[serde(default)]
    pub head_sha: String,
    pub head_branch: Option<String>,
    #[serde(default)]
    pub run_attempt: i64,
    pub workflow_name: Option<String>,
    pub runner_name: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    pub created_at: Option<String>,
}

impl From<WorkflowJobAnswer> for GithubWorkflowJob {
    /// A job of a workflow run, in our terms.
    fn from(answer: WorkflowJobAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            run_id: answer.run_id.to_string(),
            name: answer.name,
            status: answer.status,
            conclusion: answer.conclusion,
            url: answer.url,
            started_at: answer.started_at,
            completed_at: answer.completed_at,
            steps: answer
                .steps
                .into_iter()
                .map(GithubWorkflowStep::from)
                .collect(),
            head_sha: answer.head_sha,
            head_branch: answer.head_branch,
            run_attempt: answer.run_attempt,
            workflow_name: answer.workflow_name,
            runner_name: answer.runner_name,
            labels: answer.labels,
            created_at: answer.created_at,
        }
    }
}
