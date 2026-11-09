//! A repository's collaborator, as GitHub answers it.

use crate::github::types::{GithubCollaborator, GithubPermissions};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own flags, one each"
)]
pub struct PermissionsAnswer {
    #[serde(default)]
    pub admin: bool,
    #[serde(default)]
    pub maintain: bool,
    #[serde(default)]
    pub push: bool,
    #[serde(default)]
    pub triage: bool,
    #[serde(default)]
    pub pull: bool,
}

impl From<PermissionsAnswer> for GithubPermissions {
    /// What an account may do in a repository, in our terms.
    fn from(answer: PermissionsAnswer) -> Self {
        Self {
            admin: answer.admin,
            maintain: answer.maintain,
            push: answer.push,
            triage: answer.triage,
            pull: answer.pull,
        }
    }
}

#[derive(Deserialize)]
pub struct CollaboratorAnswer {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(rename = "type", default)]
    pub account_type: String,
    #[serde(default)]
    pub site_admin: bool,
    pub permissions: Option<PermissionsAnswer>,
    pub role_name: Option<String>,
}

impl From<CollaboratorAnswer> for GithubCollaborator {
    /// A repository's collaborator, in our terms.
    fn from(answer: CollaboratorAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            login: answer.login,
            avatar_url: answer.avatar_url,
            url: answer.url,
            account_type: answer.account_type,
            site_admin: answer.site_admin,
            permissions: answer.permissions.map(GithubPermissions::from),
            role_name: answer.role_name,
        }
    }
}
