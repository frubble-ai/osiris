//! `github.list_installations`: where our GitHub App is installed, of what the user can see.

use crate::ProviderError;
use crate::github::{
    Github,
    get_page::{PER_PAGE, get_page},
    transformers::installation::InstallationsPage,
    types::GithubInstallation,
};
use serde::Deserialize;
use utoipa::ToSchema;

/// No input: the installations are those the connection's user can see.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListInstallations {}

impl ListInstallations {
    /// None: a GitHub App's user token reaches what the app is installed on.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// Every installation, across pages, up to `max_items`.
    pub async fn list_installations(
        &self,
        http: &reqwest::Client,
        token: &str,
        _input: &ListInstallations,
    ) -> Result<Vec<GithubInstallation>, ProviderError> {
        let mut all: Vec<GithubInstallation> = Vec::new();
        let mut page = 1;
        loop {
            let next: InstallationsPage =
                get_page(self, http, token, "/user/installations", page).await?;
            let short = next.installations.len() < PER_PAGE;
            all.extend(next.installations.into_iter().map(GithubInstallation::from));
            if all.len() >= self.limits.max_items {
                all.truncate(self.limits.max_items);
                return Ok(all);
            }
            if short || u64::try_from(all.len()).unwrap_or(u64::MAX) >= next.total_count {
                return Ok(all);
            }
            page += 1;
        }
    }
}
