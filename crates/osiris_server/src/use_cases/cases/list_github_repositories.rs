//! `github.list_repositories`: every repository the GitHub App's installations give the connected user, each once.

use crate::use_cases::{
    UseCaseResult, UseCases, cases::run_connection_action::RunConnectionActionArgs,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::types::GithubRepository;

pub struct ListGithubRepositoriesArgs<'a> {
    /// The connection to read through.
    pub id: &'a str,
}

impl UseCases {
    pub async fn list_github_repositories(
        &self,
        args: ListGithubRepositoriesArgs<'_>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("list_github_repositories", async {
            let ctx = self.context();
            let (provider, http) = (&ctx.providers.github, &ctx.http);
            self.run_connection_action(RunConnectionActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repositories",
                call: move |token: Secret| async move {
                    provider
                        .list_repositories(http, token.expose_secret())
                        .await
                },
            })
            .await
        })
        .await
    }
}
