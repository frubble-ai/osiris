//! An issue (GitHub also answers pull requests as issues; `is_pull_request` says which).

use super::{GithubLabel, GithubMilestone, GithubReactions, GithubUser};
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
    /// Whether its conversation is locked, and why when it is: `off-topic`, `too heated`,
    /// `resolved` or `spam`.
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    /// Why it is closed or open again: `completed`, `not_planned`, `duplicate` or `reopened`.
    pub state_reason: Option<String>,
    /// How many of each reaction it has.
    pub reactions: Option<GithubReactions>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}
