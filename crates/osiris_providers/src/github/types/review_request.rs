//! A pull request after reviewers were requested from it or removed from it.

use super::{GithubPullRequest, GithubTeam, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A pull request after reviewers were requested from it or removed from it, with who its review
/// is now requested from.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubReviewRequest {
    pub pull_request: GithubPullRequest,
    pub requested_reviewers: Vec<GithubUser>,
    pub requested_teams: Vec<GithubTeam>,
}
