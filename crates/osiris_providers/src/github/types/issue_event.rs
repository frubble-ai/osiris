//! Something that happened to an issue (labelled, closed, referenced…).

use super::{GithubEventLabel, GithubEventMilestone, GithubEventRename, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Something that happened to an issue (labelled, closed, referenced…), with what it was about:
/// each detail is set only for the events that have it, as GitHub sends them.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubIssueEvent {
    pub id: String,
    /// What happened (`labeled`, `assigned`, `closed`, `renamed`…), as GitHub names it. An open
    /// string.
    pub event: String,
    pub actor: Option<GithubUser>,
    pub created_at: String,
    pub commit_id: Option<String>,
    /// The issue it happened to, in a repository's list of events.
    pub issue_number: Option<i64>,
    /// `labeled` and `unlabeled`.
    pub label: Option<GithubEventLabel>,
    /// `assigned` and `unassigned`: who was, and who did it.
    pub assignee: Option<GithubUser>,
    pub assigner: Option<GithubUser>,
    /// `milestoned` and `demilestoned`.
    pub milestone: Option<GithubEventMilestone>,
    /// `renamed`.
    pub rename: Option<GithubEventRename>,
    /// `review_requested` and `review_request_removed`.
    pub requested_reviewer: Option<GithubUser>,
    /// `closed` and `reopened`: why.
    pub state_reason: Option<String>,
    /// `locked`: why.
    pub lock_reason: Option<String>,
}
