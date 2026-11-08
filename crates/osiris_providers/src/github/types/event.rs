//! An event on GitHub's activity feed, and its parts.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The repository an event happened in.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubEventRepository {
    pub id: String,
    pub name: String,
}

/// An event on GitHub's activity feed. `payload` is GitHub's own, whose shape depends on `event_type`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubEvent {
    pub id: String,
    pub event_type: String,
    pub actor: Option<GithubUser>,
    pub repository: Option<GithubEventRepository>,
    pub public: bool,
    pub created_at: String,
    #[schema(value_type = Object)]
    pub payload: serde_json::Value,
}
