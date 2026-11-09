//! Something on an issue's timeline, as GitHub answers it.

use super::event_details::{
    EventIssueAnswer, EventLabelAnswer, EventMilestoneAnswer, EventRenameAnswer,
};
use super::user::UserAnswer;
use crate::github::types::{
    GithubEventLabel, GithubEventMilestone, GithubEventRename, GithubTimelineEvent, GithubUser,
};
use serde::Deserialize;

/// What a cross-reference points from.
#[derive(Deserialize)]
pub struct TimelineSourceAnswer {
    pub issue: Option<EventIssueAnswer>,
}

#[derive(Deserialize)]
pub struct TimelineEventAnswer {
    pub id: Option<u64>,
    #[serde(default)]
    pub event: String,
    pub actor: Option<UserAnswer>,
    pub created_at: Option<String>,
    pub commit_id: Option<String>,
    pub body: Option<String>,
    pub user: Option<UserAnswer>,
    pub html_url: Option<String>,
    pub sha: Option<String>,
    pub message: Option<String>,
    pub source: Option<TimelineSourceAnswer>,
    pub label: Option<EventLabelAnswer>,
    pub assignee: Option<UserAnswer>,
    pub milestone: Option<EventMilestoneAnswer>,
    pub rename: Option<EventRenameAnswer>,
    pub state_reason: Option<String>,
}

impl From<TimelineEventAnswer> for GithubTimelineEvent {
    /// Something on an issue's timeline, in our terms.
    fn from(answer: TimelineEventAnswer) -> Self {
        Self {
            id: answer.id.map(|id| id.to_string()),
            event: answer.event,
            actor: answer.actor.map(GithubUser::from),
            created_at: answer.created_at,
            commit_id: answer.commit_id,
            body: answer.body,
            user: answer.user.map(GithubUser::from),
            url: answer.html_url,
            sha: answer.sha,
            message: answer.message,
            source_issue_number: answer
                .source
                .and_then(|source| source.issue)
                .map(|issue| issue.number),
            label: answer.label.map(GithubEventLabel::from),
            assignee: answer.assignee.map(GithubUser::from),
            milestone: answer.milestone.map(GithubEventMilestone::from),
            rename: answer.rename.map(GithubEventRename::from),
            state_reason: answer.state_reason,
        }
    }
}
