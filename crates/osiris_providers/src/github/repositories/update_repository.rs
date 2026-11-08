//! `github.update_repository`: a repository's settings and metadata changed.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, repositories::trimmed::trimmed, request::send,
        transformers::repository::RepositoryAnswer, types::GithubRepository,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own settings, one each"
)]
pub struct UpdateRepository {
    pub owner: String,
    pub repo: String,
    /// A new name for the repository.
    pub name: Option<String>,
    pub description: Option<String>,
    /// A URL with more about the repository.
    pub homepage: Option<String>,
    pub private: Option<bool>,
    /// `public` or `private`.
    pub visibility: Option<String>,
    pub default_branch: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_discussions: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    /// Whether a pull request's head branch is deleted once it is merged.
    pub delete_branch_on_merge: Option<bool>,
    pub archived: Option<bool>,
}

impl UpdateRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own settings, one each"
)]
struct Body<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_issues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_projects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_wiki: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_discussions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_squash_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_merge_commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_rebase_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_auto_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_branch_on_merge: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    archived: Option<bool>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}`.
    pub async fn update_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateRepository,
    ) -> Result<GithubRepository, ProviderError> {
        let path = format!(
            "/repos/{}/{}",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = Body {
            name: input.name.as_deref(),
            description: input.description.as_deref(),
            homepage: input.homepage.as_deref(),
            private: input.private,
            visibility: trimmed(input.visibility.as_deref()),
            default_branch: trimmed(input.default_branch.as_deref()),
            has_issues: input.has_issues,
            has_projects: input.has_projects,
            has_wiki: input.has_wiki,
            has_discussions: input.has_discussions,
            allow_squash_merge: input.allow_squash_merge,
            allow_merge_commit: input.allow_merge_commit,
            allow_rebase_merge: input.allow_rebase_merge,
            allow_auto_merge: input.allow_auto_merge,
            delete_branch_on_merge: input.delete_branch_on_merge,
            archived: input.archived,
        };
        let answer: RepositoryAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
