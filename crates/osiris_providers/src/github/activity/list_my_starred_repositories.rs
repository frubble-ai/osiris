//! `github.list_my_starred_repositories`: the repositories the authenticated user starred, a page
//! at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, request::send, transformers::repository::RepositoryAnswer,
        types::GithubRepository,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListMyStarredRepositories {
    /// `created` (when the star was given, GitHub's default) or `updated` (when the repository was
    /// last pushed to).
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default).
    pub direction: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListMyStarredRepositories {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /user/starred`.
    pub async fn list_my_starred_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListMyStarredRepositories,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let mut query = Vec::new();
        if let Some(sort) = &input.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(direction) = &input.direction {
            query.push(("direction", direction.clone()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<RepositoryAnswer> = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/user/starred",
            &query,
            None,
        )
        .await?;
        Ok(answer.into_iter().map(GithubRepository::from).collect())
    }
}
