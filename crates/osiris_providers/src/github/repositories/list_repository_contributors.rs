//! `github.list_repository_contributors`: who contributed to a repository, the most contributions
//! first, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::contributor::ContributorAnswer, types::GithubContributor,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryContributors {
    pub owner: String,
    pub repo: String,
    /// Whether to include contributors whose commits match no GitHub account.
    pub anon: Option<bool>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryContributors {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/contributors`. GitHub answers nothing (a 204) for an empty
    /// repository, which is an empty list.
    pub async fn list_repository_contributors(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryContributors,
    ) -> Result<Vec<GithubContributor>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/contributors",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = Vec::new();
        if let Some(anon) = input.anon {
            query.push(("anon", anon.to_string()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Option<Vec<ContributorAnswer>> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer
            .unwrap_or_default()
            .into_iter()
            .map(GithubContributor::from)
            .collect())
    }
}
