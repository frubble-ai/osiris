//! Something on an issue's timeline, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubTimelineEvent, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TimelineEventAnswer {
    pub id: Option<u64>,
    #[serde(default)]
    pub event: String,
    pub actor: Option<UserAnswer>,
    pub created_at: Option<String>,
    pub commit_id: Option<String>,
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
        }
    }
}
