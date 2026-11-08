//! An account's permission in a repository.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An account's permission in a repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubRepositoryPermission {
    pub permission: String,
    pub role_name: Option<String>,
    pub user: Option<GithubUser>,
}
