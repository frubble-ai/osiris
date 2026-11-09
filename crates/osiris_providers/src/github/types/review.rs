//! A review of a pull request.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A review of a pull request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubReview {
    pub id: String,
    pub user: Option<GithubUser>,
    pub body: Option<String>,
    pub state: String,
    pub url: String,
    pub commit_id: Option<String>,
    pub submitted_at: Option<String>,
    /// How the author is related to the repository (`OWNER`, `MEMBER`, `CONTRIBUTOR`, `NONE`…),
    /// as GitHub names it. An open string.
    pub author_association: String,
}
