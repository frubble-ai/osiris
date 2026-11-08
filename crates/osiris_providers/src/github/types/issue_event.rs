//! Something that happened to an issue (labelled, closed, referenced…).

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Something that happened to an issue (labelled, closed, referenced…).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubIssueEvent {
    pub id: String,
    pub event: String,
    pub actor: Option<GithubUser>,
    pub created_at: String,
    pub commit_id: Option<String>,
}
