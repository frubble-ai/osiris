//! An account's permission in a repository, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubRepositoryPermission, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepositoryPermissionAnswer {
    #[serde(default)]
    pub permission: String,
    pub role_name: Option<String>,
    pub user: Option<UserAnswer>,
}

impl From<RepositoryPermissionAnswer> for GithubRepositoryPermission {
    /// An account's permission in a repository, in our terms.
    fn from(answer: RepositoryPermissionAnswer) -> Self {
        Self {
            permission: answer.permission,
            role_name: answer.role_name,
            user: answer.user.map(GithubUser::from),
        }
    }
}
