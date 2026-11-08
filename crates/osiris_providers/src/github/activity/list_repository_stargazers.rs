//! `github.list_repository_stargazers`: the users who starred a repository, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::user::UserAnswer, types::GithubUser,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryStargazers {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryStargazers {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/stargazers`.
    pub async fn list_repository_stargazers(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryStargazers,
    ) -> Result<Vec<GithubUser>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/stargazers",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<UserAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubUser::from).collect())
    }
}
