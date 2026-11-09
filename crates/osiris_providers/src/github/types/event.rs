//! An event on GitHub's activity feed, and its parts.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Who did an event, or the organization it happened in: GitHub names an account on its activity
/// feeds by less than it does elsewhere, with no page link and no account type.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubEventAccount {
    pub id: String,
    pub login: String,
    pub display_login: Option<String>,
    pub avatar_url: String,
}

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
    pub actor: Option<GithubEventAccount>,
    pub repository: Option<GithubEventRepository>,
    pub organization: Option<GithubEventAccount>,
    pub public: bool,
    pub created_at: String,
    #[schema(value_type = Object)]
    pub payload: serde_json::Value,
}
