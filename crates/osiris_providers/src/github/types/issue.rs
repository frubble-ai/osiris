//! An issue (GitHub also answers pull requests as issues; `is_pull_request` says which).

use super::{GithubLabel, GithubMilestone, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An issue (GitHub also answers pull requests as issues; `is_pull_request` says which).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubIssue {
    pub id: String,
    pub number: i64,
    pub title: String,
    pub state: String,
    pub url: String,
    pub body: Option<String>,
    pub comments: i64,
    pub user: Option<GithubUser>,
    pub assignees: Vec<GithubUser>,
    pub labels: Vec<GithubLabel>,
    pub milestone: Option<GithubMilestone>,
    pub is_pull_request: bool,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}
