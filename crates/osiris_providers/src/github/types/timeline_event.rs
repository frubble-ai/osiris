//! Something on an issue's timeline: an event, a comment, a commit or a cross-reference.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Something on an issue's timeline: an event, a comment, a commit or a cross-reference. Unlike an
/// issue event, a timeline entry may have no id, no actor and no time (a commit or a
/// cross-reference has none of its own).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubTimelineEvent {
    pub id: Option<String>,
    /// What happened (`commented`, `committed`, `labeled`, `cross-referenced`…), as GitHub names
    /// it. An open string.
    pub event: String,
    pub actor: Option<GithubUser>,
    pub created_at: Option<String>,
    pub commit_id: Option<String>,
}
