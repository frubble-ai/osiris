//! Someone who contributed to a repository.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Someone who contributed to a repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubContributor {
    pub id: String,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub account_type: String,
    pub site_admin: bool,
    pub contributions: i64,
}
