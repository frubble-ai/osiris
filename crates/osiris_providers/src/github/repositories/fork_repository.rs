//! `github.fork_repository`: a fork of a repository. GitHub forks in the background, so the
//! repository answered may not be ready at once.

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
pub struct ForkRepository {
    pub owner: String,
    pub repo: String,
    /// The organization to fork into; the authenticated user's account when left out.
    pub organization: Option<String>,
    /// A name for the fork; the repository's own when left out.
    pub name: Option<String>,
    /// Whether the fork has only the default branch.
    pub default_branch_only: Option<bool>,
}

impl ForkRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_branch_only: Option<bool>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/forks`.
    pub async fn fork_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ForkRepository,
    ) -> Result<GithubRepository, ProviderError> {
        let path = format!(
            "/repos/{}/{}/forks",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = Body {
            organization: trimmed(input.organization.as_deref()),
            name: trimmed(input.name.as_deref()),
            default_branch_only: input.default_branch_only,
        };
        let answer: RepositoryAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
