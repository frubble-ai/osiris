//! A comment on a commit.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A comment on a commit.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommitComment {
    pub id: String,
    pub body: String,
    pub url: String,
    pub path: Option<String>,
    pub position: Option<i64>,
    pub user: Option<GithubUser>,
    pub created_at: String,
    pub updated_at: String,
}
