//! A run of a workflow.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A run of a workflow.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubWorkflowRun {
    pub id: String,
    pub name: Option<String>,
    pub display_title: String,
    pub workflow_id: String,
    pub event: String,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub url: String,
    pub run_number: i64,
    pub created_at: String,
    pub updated_at: String,
}
