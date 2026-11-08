//! A comment on an issue or a pull request.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A comment on an issue or a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubIssueComment {
    pub id: String,
    pub url: String,
    pub body: String,
    pub user: Option<GithubUser>,
    pub created_at: String,
    pub updated_at: String,
}
