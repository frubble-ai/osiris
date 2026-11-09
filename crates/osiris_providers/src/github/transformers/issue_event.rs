//! Something that happened to an issue (labelled, closed, referenced…), as GitHub answers it.

use super::event_details::{
    EventIssueAnswer, EventLabelAnswer, EventMilestoneAnswer, EventRenameAnswer,
};
use super::user::UserAnswer;
use crate::github::types::{
    GithubEventLabel, GithubEventMilestone, GithubEventRename, GithubIssueEvent, GithubUser,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IssueEventAnswer {
    pub id: u64,
    #[serde(default)]
    pub event: String,
    pub actor: Option<UserAnswer>,
    #[serde(default)]
    pub created_at: String,
    pub commit_id: Option<String>,
    pub issue: Option<EventIssueAnswer>,
    pub label: Option<EventLabelAnswer>,
    pub assignee: Option<UserAnswer>,
    pub assigner: Option<UserAnswer>,
    pub milestone: Option<EventMilestoneAnswer>,
    pub rename: Option<EventRenameAnswer>,
    pub requested_reviewer: Option<UserAnswer>,
    pub state_reason: Option<String>,
    pub lock_reason: Option<String>,
}

impl From<IssueEventAnswer> for GithubIssueEvent {
    /// Something that happened to an issue (labelled, closed, referenced…), in our terms.
    fn from(answer: IssueEventAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            event: answer.event,
            actor: answer.actor.map(GithubUser::from),
            created_at: answer.created_at,
            commit_id: answer.commit_id,
            issue_number: answer.issue.map(|issue| issue.number),
            label: answer.label.map(GithubEventLabel::from),
            assignee: answer.assignee.map(GithubUser::from),
            assigner: answer.assigner.map(GithubUser::from),
            milestone: answer.milestone.map(GithubEventMilestone::from),
            rename: answer.rename.map(GithubEventRename::from),
            requested_reviewer: answer.requested_reviewer.map(GithubUser::from),
            state_reason: answer.state_reason,
            lock_reason: answer.lock_reason,
        }
    }
}
