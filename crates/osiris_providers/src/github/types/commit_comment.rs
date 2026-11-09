//! A comment on a commit.

use super::{GithubReactions, GithubUser};
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
    pub commit_id: String,
    pub line: Option<i64>,
    /// How the author is related to the repository (`OWNER`, `MEMBER`, `CONTRIBUTOR`, `NONE`…),
    /// as GitHub names it. An open string.
    pub author_association: String,
    pub reactions: Option<GithubReactions>,
}
