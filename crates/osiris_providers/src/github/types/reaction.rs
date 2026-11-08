//! A reaction to an issue or a comment.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A reaction to an issue or a comment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubReaction {
    pub id: String,
    pub content: String,
    pub user: Option<GithubUser>,
    pub created_at: String,
}
