//! `github.list_installations`: where the GitHub App is installed, of what the connected user can see.

use crate::use_cases::{
    UseCaseResult, UseCases, cases::run_connection_action::RunConnectionActionArgs,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::types::GithubInstallation;

pub struct ListGithubInstallationsArgs<'a> {
    /// The connection to read through.
    pub id: &'a str,
}

impl UseCases {
    pub async fn list_github_installations(
        &self,
        args: ListGithubInstallationsArgs<'_>,
    ) -> UseCaseResult<Vec<GithubInstallation>> {
        self.run("list_github_installations", async {
            let ctx = self.context();
            let (provider, http) = (&ctx.providers.github, &ctx.http);
            self.run_connection_action(RunConnectionActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_installations",
                call: move |token: Secret| async move {
                    provider
                        .list_installations(http, token.expose_secret())
                        .await
                },
            })
            .await
        })
        .await
    }
}
