//! A repository.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubRepository {
    pub id: String,
    pub name: String,
    /// `owner/name`.
    pub full_name: String,
    pub private: bool,
    /// Its page on GitHub.
    pub url: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub description: Option<String>,
    pub default_branch: String,
    /// `public`, `private` or `internal` today. An open string.
    pub visibility: String,
    pub fork: bool,
    pub owner: GithubUser,
}
