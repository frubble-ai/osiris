//! A job of a workflow run, and its parts.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A step of a job.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubWorkflowStep {
    pub number: i64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// A job of a workflow run.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubWorkflowJob {
    pub id: String,
    pub run_id: String,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub url: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub steps: Vec<GithubWorkflowStep>,
    pub head_sha: String,
    pub head_branch: Option<String>,
    pub run_attempt: i64,
    pub workflow_name: Option<String>,
    pub runner_name: Option<String>,
    pub labels: Vec<String>,
    pub created_at: Option<String>,
}
