//! Something that happened to an issue (labelled, closed, referenced…), as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubIssueEvent, GithubUser};
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
        }
    }
}
