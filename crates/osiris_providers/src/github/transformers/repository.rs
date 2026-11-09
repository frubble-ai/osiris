//! A repository, as GitHub answers it.

use super::{collaborator::PermissionsAnswer, user::UserAnswer};
use crate::github::types::{GithubLicense, GithubPermissions, GithubRepository, GithubUser};
use serde::Deserialize;

/// A page of `GET /user/installations/{id}/repositories`.
#[derive(Deserialize)]
pub struct RepositoriesPage {
    pub total_count: u64,
    pub repositories: Vec<RepositoryAnswer>,
}

#[derive(Deserialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own flags, one each"
)]
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
    homepage: Option<String>,
    language: Option<String>,
    #[serde(default)]
    topics: Vec<String>,
    license: Option<LicenseAnswer>,
    #[serde(default)]
    archived: bool,
    #[serde(default)]
    disabled: bool,
    #[serde(default)]
    is_template: bool,
    #[serde(default)]
    has_issues: bool,
    #[serde(default)]
    has_projects: bool,
    #[serde(default)]
    has_wiki: bool,
    #[serde(default)]
    has_pages: bool,
    #[serde(default)]
    has_discussions: bool,
    #[serde(default)]
    stargazers_count: i64,
    #[serde(default)]
    watchers_count: i64,
    #[serde(default)]
    forks_count: i64,
    #[serde(default)]
    open_issues_count: i64,
    #[serde(default)]
    size: i64,
    created_at: Option<String>,
    updated_at: Option<String>,
    pushed_at: Option<String>,
    permissions: Option<PermissionsAnswer>,
    allow_merge_commit: Option<bool>,
    allow_squash_merge: Option<bool>,
    allow_rebase_merge: Option<bool>,
    allow_auto_merge: Option<bool>,
    delete_branch_on_merge: Option<bool>,
    subscribers_count: Option<i64>,
}

#[derive(Deserialize)]
pub struct LicenseAnswer {
    #[serde(default)]
    key: String,
    #[serde(default)]
    name: String,
    spdx_id: Option<String>,
}

impl From<LicenseAnswer> for GithubLicense {
    /// The license GitHub found, in our terms.
    fn from(answer: LicenseAnswer) -> Self {
        Self {
            key: answer.key,
            name: answer.name,
            spdx_id: answer.spdx_id,
        }
    }
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
            homepage: answer.homepage,
            language: answer.language,
            topics: answer.topics,
            license: answer.license.map(GithubLicense::from),
            archived: answer.archived,
            disabled: answer.disabled,
            is_template: answer.is_template,
            has_issues: answer.has_issues,
            has_projects: answer.has_projects,
            has_wiki: answer.has_wiki,
            has_pages: answer.has_pages,
            has_discussions: answer.has_discussions,
            stargazers_count: answer.stargazers_count,
            watchers_count: answer.watchers_count,
            forks_count: answer.forks_count,
            open_issues_count: answer.open_issues_count,
            size: answer.size,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            pushed_at: answer.pushed_at,
            permissions: answer.permissions.map(GithubPermissions::from),
            allow_merge_commit: answer.allow_merge_commit,
            allow_squash_merge: answer.allow_squash_merge,
            allow_rebase_merge: answer.allow_rebase_merge,
            allow_auto_merge: answer.allow_auto_merge,
            delete_branch_on_merge: answer.delete_branch_on_merge,
            subscribers_count: answer.subscribers_count,
        }
    }
}
