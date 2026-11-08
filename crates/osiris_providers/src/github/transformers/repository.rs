//! A repository, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubRepository, GithubUser};
use serde::Deserialize;

/// A page of `GET /user/installations/{id}/repositories`.
#[derive(Deserialize)]
pub struct RepositoriesPage {
    pub total_count: u64,
    pub repositories: Vec<RepositoryAnswer>,
}

#[derive(Deserialize)]
pub struct RepositoryAnswer {
    pub id: u64,
    name: String,
    full_name: String,
    private: bool,
    html_url: String,
    #[serde(default)]
    clone_url: String,
    #[serde(default)]
    ssh_url: String,
    description: Option<String>,
    #[serde(default)]
    default_branch: String,
    #[serde(default)]
    visibility: String,
    #[serde(default)]
    fork: bool,
    owner: UserAnswer,
}

impl From<RepositoryAnswer> for GithubRepository {
    /// The repository in our terms.
    fn from(answer: RepositoryAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            full_name: answer.full_name,
            private: answer.private,
            url: answer.html_url,
            clone_url: answer.clone_url,
            ssh_url: answer.ssh_url,
            description: answer.description,
            default_branch: answer.default_branch,
            visibility: answer.visibility,
            fork: answer.fork,
            owner: GithubUser::from(answer.owner),
        }
    }
}
