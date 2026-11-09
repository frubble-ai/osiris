//! A run of a workflow, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubUser, GithubWorkflowRun};
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
    #[serde(default)]
    pub run_attempt: i64,
    pub run_started_at: Option<String>,
    pub actor: Option<UserAnswer>,
    pub triggering_actor: Option<UserAnswer>,
    #[serde(default)]
    pub path: String,
    pub check_suite_id: Option<u64>,
    #[serde(default)]
    pub pull_requests: Vec<RunPullRequestAnswer>,
}

/// A pull request a run is for, of which we keep the number.
#[derive(Deserialize)]
pub struct RunPullRequestAnswer {
    pub number: i64,
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
            run_attempt: answer.run_attempt,
            run_started_at: answer.run_started_at,
            actor: answer.actor.map(GithubUser::from),
            triggering_actor: answer.triggering_actor.map(GithubUser::from),
            path: answer.path,
            check_suite_id: answer.check_suite_id.map(|id| id.to_string()),
            pull_request_numbers: answer
                .pull_requests
                .into_iter()
                .map(|pull_request| pull_request.number)
                .collect(),
        }
    }
}
