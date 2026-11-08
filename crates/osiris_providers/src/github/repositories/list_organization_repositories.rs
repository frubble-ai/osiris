//! `github.list_organization_repositories`: an organization's repositories, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::repository::RepositoryAnswer, trimmed::trimmed, types::GithubRepository,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListOrganizationRepositories {
    /// The organization's login.
    pub org: String,
    /// `all`, `public`, `private`, `forks`, `sources` or `member`.
    #[serde(rename = "type")]
    pub repository_type: Option<String>,
    /// `created`, `updated`, `pushed` or `full_name`.
    pub sort: Option<String>,
    /// `asc` or `desc`.
    pub direction: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListOrganizationRepositories {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /orgs/{org}/repos`.
    pub async fn list_organization_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListOrganizationRepositories,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let path = format!("/orgs/{}/repos", path_segment(&input.org));
        let mut query = Vec::new();
        for (name, value) in [
            ("type", &input.repository_type),
            ("sort", &input.sort),
            ("direction", &input.direction),
        ] {
            if let Some(value) = trimmed(value.as_deref()) {
                query.push((name, value.to_owned()));
            }
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<RepositoryAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubRepository::from).collect())
    }
}
