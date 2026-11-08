//! An event on GitHub's activity feed, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubEvent, GithubEventRepository, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventRepositoryAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

impl From<EventRepositoryAnswer> for GithubEventRepository {
    /// The repository an event happened in, in our terms.
    fn from(answer: EventRepositoryAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
        }
    }
}

#[derive(Deserialize)]
pub struct EventAnswer {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "type", default)]
    pub event_type: String,
    pub actor: Option<UserAnswer>,
    #[serde(rename = "repo")]
    pub repository: Option<EventRepositoryAnswer>,
    #[serde(default)]
    pub public: bool,
    #[serde(default)]
    pub created_at: String,
    pub payload: serde_json::Value,
}

impl From<EventAnswer> for GithubEvent {
    /// An event on GitHub's activity feed, in our terms.
    fn from(answer: EventAnswer) -> Self {
        Self {
            id: answer.id,
            event_type: answer.event_type,
            actor: answer.actor.map(GithubUser::from),
            repository: answer.repository.map(GithubEventRepository::from),
            public: answer.public,
            created_at: answer.created_at,
            payload: answer.payload,
        }
    }
}
