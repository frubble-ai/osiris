//! Something on an issue's timeline: an event, a comment, a commit or a cross-reference.

use super::{GithubEventLabel, GithubEventMilestone, GithubEventRename, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Something on an issue's timeline: an event, a comment, a commit or a cross-reference. Unlike an
/// issue event, a timeline entry may have no id, no actor and no time (a commit or a
/// cross-reference has none of its own). Each detail is set only for the entries that have it, as
/// GitHub sends them.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubTimelineEvent {
    pub id: Option<String>,
    /// What happened (`commented`, `committed`, `labeled`, `cross-referenced`…), as GitHub names
    /// it. An open string.
    pub event: String,
    pub actor: Option<GithubUser>,
    pub created_at: Option<String>,
    pub commit_id: Option<String>,
    /// `commented`: the comment, its author and its page.
    pub body: Option<String>,
    pub user: Option<GithubUser>,
    pub url: Option<String>,
    /// `committed`: the commit, its message and who wrote and committed it, and when.
    pub sha: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub authored_at: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub committed_at: Option<String>,
    /// `reviewed`: how the review ended (`approved`, `changes_requested`, `commented`), and when.
    pub state: Option<String>,
    pub submitted_at: Option<String>,
    /// `cross-referenced`: the issue or pull request that mentions this one.
    pub source_issue_number: Option<i64>,
    /// `labeled` and `unlabeled`.
    pub label: Option<GithubEventLabel>,
    /// `assigned` and `unassigned`.
    pub assignee: Option<GithubUser>,
    /// `milestoned` and `demilestoned`.
    pub milestone: Option<GithubEventMilestone>,
    /// `renamed`.
    pub rename: Option<GithubEventRename>,
    /// `closed` and `reopened`: why.
    pub state_reason: Option<String>,
}
