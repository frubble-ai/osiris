//! `github.list_repositories`: every repository the GitHub App's installations give the connected user, each once.

use crate::core::{Core, CoreResult};
use osiris_common::secret::ExposeSecret;
use osiris_providers::github::types::GithubRepository;

impl Core {
    pub async fn github_list_repositories(&self, id: &str) -> CoreResult<Vec<GithubRepository>> {
        let (provider, http) = (&self.providers.github, &self.http);
        self.with_token(
            id,
            "github",
            "github.list_repositories",
            move |token| async move {
                provider
                    .list_repositories(http, token.expose_secret())
                    .await
            },
        )
        .await
    }
}
