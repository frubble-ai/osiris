//! `github.list_repositories`: every repository our GitHub App's installations give the user, or,
//! on an installation connection, every repository that installation was given.

use super::list_installations::ListInstallations;
use crate::github::{
    Github,
    get_page::{PER_PAGE, get_page},
    transformers::repository::RepositoriesPage,
    types::GithubRepository,
};
use crate::{ActingAs, ProviderError};
use futures::{StreamExt, TryStreamExt, stream};
use serde::Deserialize;
use std::collections::BTreeMap;
use utoipa::ToSchema;

/// No input: the repositories are those the app's installations give the connection's user.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositories {}

impl ListRepositories {
    /// None: a GitHub App's user token reaches what the app is installed on.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// Every repository of every installation, each once, ordered by id: installations read a few
    /// at once, and at most `max_items` repositories. An installation's own token reads its
    /// repositories directly (`GET /installation/repositories`).
    pub async fn list_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        _input: &ListRepositories,
        acting: ActingAs,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        if acting == ActingAs::Installation {
            return self
                .repositories_at(http, token, "/installation/repositories")
                .await;
        }
        let installations = self
            .list_installations(http, token, &ListInstallations {})
            .await?;
        let mut pages = stream::iter(installations)
            .map(|installation| self.installation_repositories(http, token, installation.id))
            .buffer_unordered(self.limits.concurrency.max(1));
        let mut by_id = BTreeMap::new();
        // Installations are read until the map holds `max_items`; the rest are never fetched.
        while by_id.len() < self.limits.max_items
            && let Some(page) = pages.try_next().await?
        {
            for repository in page {
                by_id.insert(repository.id.clone(), repository);
            }
        }
        Ok(by_id.into_values().take(self.limits.max_items).collect())
    }

    /// One installation's repositories, as its user sees them.
    async fn installation_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        installation: String,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let path = format!("/user/installations/{installation}/repositories");
        self.repositories_at(http, token, &path).await
    }

    /// The repositories GitHub pages at `path`, page after page, up to `max_items`.
    pub(crate) async fn repositories_at(
        &self,
        http: &reqwest::Client,
        token: &str,
        path: &str,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let mut repositories = Vec::new();
        let mut page = 1;
        loop {
            let next: RepositoriesPage = get_page(self, http, token, path, page).await?;
            let short = next.repositories.len() < PER_PAGE;
            repositories.extend(next.repositories.into_iter().map(GithubRepository::from));
            let seen = u64::try_from(repositories.len()).unwrap_or(u64::MAX);
            if short || seen >= next.total_count || repositories.len() >= self.limits.max_items {
                return Ok(repositories);
            }
            page += 1;
        }
    }
}
