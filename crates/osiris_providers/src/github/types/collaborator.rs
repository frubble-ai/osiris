//! A repository's collaborator, and its parts.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// What an account may do in a repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own flags, one each"
)]
pub struct GithubPermissions {
    pub admin: bool,
    pub maintain: bool,
    pub push: bool,
    pub triage: bool,
    pub pull: bool,
}

/// A repository's collaborator.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCollaborator {
    pub id: String,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub account_type: String,
    pub site_admin: bool,
    pub permissions: Option<GithubPermissions>,
    pub role_name: Option<String>,
}
