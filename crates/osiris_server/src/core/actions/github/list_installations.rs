//! `github.list_installations`: where the GitHub App is installed, of what the connected user can see.

use crate::core::{Core, CoreResult};
use osiris_common::secret::ExposeSecret;
use osiris_providers::github::types::GithubInstallation;

impl Core {
    pub async fn github_list_installations(&self, id: &str) -> CoreResult<Vec<GithubInstallation>> {
        let (provider, http) = (&self.providers.github, &self.http);
        self.with_token(
            id,
            "github",
            "github.list_installations",
            move |token| async move {
                provider
                    .list_installations(http, token.expose_secret())
                    .await
            },
        )
        .await
    }
}
