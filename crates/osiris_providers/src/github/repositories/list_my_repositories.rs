//! `github.list_my_repositories`: the repositories the authenticated user can see, a page at a time,
//! or, on an installation connection, the repositories that installation was given, as
//! `OpenConnector` lists them there.

use crate::{
    ActingAs, ProviderError,
    github::{
        Github,
        paging::paging,
        request::send,
        transformers::repository::{RepositoriesPage, RepositoryAnswer},
        trimmed::trimmed,
        types::GithubRepository,
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
    /// `GET /user/repos`; for an installation, `GET /installation/repositories`, which takes only
    /// the page.
    pub async fn list_my_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListMyRepositories,
        acting: ActingAs,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        if acting == ActingAs::Installation {
            return self.installation_page(http, token, input).await;
        }
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

    /// One page of the installation's repositories. A user's filter can't apply to them, so one
    /// given is refused rather than ignored.
    async fn installation_page(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListMyRepositories,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        for (name, value) in [
            ("visibility", &input.visibility),
            ("sort", &input.sort),
            ("direction", &input.direction),
        ] {
            if value.is_some() {
                return Err(ProviderError::invalid(
                    "github",
                    format!(
                        "{name} can't be used on an installation connection; leave it out to list the installation's repositories"
                    ),
                ));
            }
        }
        let query = paging(input.per_page, input.page);
        let answer: RepositoriesPage = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/installation/repositories",
            &query,
            None,
        )
        .await?;
        Ok(answer
            .repositories
            .into_iter()
            .map(GithubRepository::from)
            .collect())
    }
}
