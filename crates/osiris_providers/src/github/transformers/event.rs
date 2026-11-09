//! An event on GitHub's activity feed, as GitHub answers it.

use crate::github::types::{GithubEvent, GithubEventAccount, GithubEventRepository};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventAccountAnswer {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    pub display_login: Option<String>,
    #[serde(default)]
    pub avatar_url: String,
}

impl From<EventAccountAnswer> for GithubEventAccount {
    /// Who did an event, or the organization it happened in, in our terms.
    fn from(answer: EventAccountAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            login: answer.login,
            display_login: answer.display_login,
            avatar_url: answer.avatar_url,
        }
    }
}

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
    pub actor: Option<EventAccountAnswer>,
    #[serde(rename = "repo")]
    pub repository: Option<EventRepositoryAnswer>,
    #[serde(rename = "org")]
    pub organization: Option<EventAccountAnswer>,
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
            actor: answer.actor.map(GithubEventAccount::from),
            repository: answer.repository.map(GithubEventRepository::from),
            organization: answer.organization.map(GithubEventAccount::from),
            public: answer.public,
            created_at: answer.created_at,
            payload: answer.payload,
        }
    }
}
