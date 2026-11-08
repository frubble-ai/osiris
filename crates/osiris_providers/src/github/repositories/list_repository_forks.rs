//! `github.list_repository_forks`: a repository's forks, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, repositories::trimmed::trimmed,
        request::send, transformers::repository::RepositoryAnswer, types::GithubRepository,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryForks {
    pub owner: String,
    pub repo: String,
    /// `newest`, `oldest`, `stargazers` or `watchers`.
    pub sort: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryForks {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/forks`.
    pub async fn list_repository_forks(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryForks,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/forks",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = Vec::new();
        if let Some(sort) = trimmed(input.sort.as_deref()) {
            query.push(("sort", sort.to_owned()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<RepositoryAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubRepository::from).collect())
    }
}
