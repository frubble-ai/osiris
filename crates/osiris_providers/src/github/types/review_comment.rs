//! A comment on a line of a pull request's diff.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A comment on a line of a pull request's diff.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubReviewComment {
    pub id: String,
    pub path: String,
    pub body: String,
    pub user: Option<GithubUser>,
    pub commit_id: String,
    pub original_commit_id: String,
    pub diff_hunk: String,
    pub url: String,
    pub line: Option<i64>,
    pub original_line: Option<i64>,
    pub start_line: Option<i64>,
    pub side: Option<String>,
    pub start_side: Option<String>,
    pub in_reply_to_id: Option<String>,
    pub pull_request_review_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
