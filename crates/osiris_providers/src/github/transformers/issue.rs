//! An issue (GitHub also answers pull requests as issues; `is_pull_request` says which), as GitHub answers it.

use super::label::LabelAnswer;
use super::milestone::MilestoneAnswer;
use super::reactions::ReactionsAnswer;
use super::user::UserAnswer;
use crate::github::types::{
    GithubIssue, GithubLabel, GithubMilestone, GithubReactions, GithubUser,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IssueAnswer {
    pub id: u64,
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub body: Option<String>,
    #[serde(default)]
    pub comments: i64,
    pub user: Option<UserAnswer>,
    #[serde(default)]
    pub assignees: Vec<UserAnswer>,
    #[serde(default)]
    pub labels: Vec<LabelAnswer>,
    pub milestone: Option<MilestoneAnswer>,
    #[serde(rename = "pull_request")]
    pub is_pull_request: Option<serde_json::Value>,
    #[serde(default)]
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub state_reason: Option<String>,
    pub reactions: Option<ReactionsAnswer>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    pub closed_at: Option<String>,
}

impl From<IssueAnswer> for GithubIssue {
    /// An issue (GitHub also answers pull requests as issues; `is_pull_request` says which), in our terms.
    fn from(answer: IssueAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            number: answer.number,
            title: answer.title,
            state: answer.state,
            url: answer.url,
            body: answer.body,
            comments: answer.comments,
            user: answer.user.map(GithubUser::from),
            assignees: answer.assignees.into_iter().map(GithubUser::from).collect(),
            labels: answer.labels.into_iter().map(GithubLabel::from).collect(),
            milestone: answer.milestone.map(GithubMilestone::from),
            is_pull_request: answer.is_pull_request.is_some(),
            locked: answer.locked,
            active_lock_reason: answer.active_lock_reason,
            state_reason: answer.state_reason,
            reactions: answer.reactions.map(GithubReactions::from),
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            closed_at: answer.closed_at,
        }
    }
}
