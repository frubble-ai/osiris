//! `github.list_my_repositories`: the repositories the authenticated user can see, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, request::send, transformers::repository::RepositoryAnswer,
        trimmed::trimmed, types::GithubRepository,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListMyRepositories {
    /// `all`, `public` or `private`.
    pub visibility: Option<String>,
    /// `created`, `updated`, `pushed` or `full_name`.
    pub sort: Option<String>,
    /// `asc` or `desc`.
    pub direction: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListMyRepositories {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /user/repos`.
    pub async fn list_my_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListMyRepositories,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let mut query = Vec::new();
        for (name, value) in [
            ("visibility", &input.visibility),
            ("sort", &input.sort),
            ("direction", &input.direction),
        ] {
            if let Some(value) = trimmed(value.as_deref()) {
                query.push((name, value.to_owned()));
            }
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<RepositoryAnswer> =
            send::<_, ()>(self, http, token, Method::GET, "/user/repos", &query, None).await?;
        Ok(answer.into_iter().map(GithubRepository::from).collect())
    }
}
