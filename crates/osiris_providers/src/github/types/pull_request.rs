//! A pull request, and its parts.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The branch a pull request comes from (`head`) or goes into (`base`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubPullRequestBranch {
    pub ref_name: String,
    pub sha: String,
    pub label: String,
    pub repository_full_name: Option<String>,
}

/// A pull request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubPullRequest {
    pub id: String,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub body: Option<String>,
    pub url: String,
    pub draft: bool,
    pub merged: bool,
    pub user: Option<GithubUser>,
    pub head: GithubPullRequestBranch,
    pub base: GithubPullRequestBranch,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
}
