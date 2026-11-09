//! A pull request, and its parts.

use super::{GithubLabel, GithubMilestone, GithubUser};
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
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own flags, one each"
)]
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
    pub labels: Vec<GithubLabel>,
    pub assignees: Vec<GithubUser>,
    pub milestone: Option<GithubMilestone>,
    /// Who is asked to review it: people, and teams by their slug.
    pub requested_reviewers: Vec<GithubUser>,
    pub requested_teams: Vec<String>,
    /// Whether it can be merged now, as GitHub has worked it out: only reading one pull request
    /// answers these (a list leaves them null), and `mergeable` is null while GitHub is still
    /// working it out.
    pub mergeable: Option<bool>,
    /// `clean`, `blocked`, `behind`, `dirty`, `unstable`, `draft` or `unknown`. An open string.
    pub mergeable_state: Option<String>,
    /// The commit its merge made, or the test merge GitHub keeps while it is open.
    pub merge_commit_sha: Option<String>,
    pub maintainer_can_modify: Option<bool>,
    /// Whether it merges by itself once its checks pass.
    pub auto_merge: bool,
    /// Its size and conversation, from reading one pull request; a list leaves them null.
    pub commits: Option<i64>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub changed_files: Option<i64>,
    pub comments: Option<i64>,
    pub review_comments: Option<i64>,
    /// Whether its conversation is locked, and why when it is.
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub merged_by: Option<GithubUser>,
    /// Whether it can be rebased onto its base; null while GitHub is still working it out.
    pub rebaseable: Option<bool>,
    /// How the author is related to the repository (`OWNER`, `MEMBER`, `CONTRIBUTOR`, `NONE`…),
    /// as GitHub names it. An open string.
    pub author_association: String,
}
