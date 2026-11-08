//! `github.list_installations`: where the GitHub App is installed, of what the connected user can see.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{installations::ListInstallations, types::GithubInstallation};

impl UseCases {
    /// Runs `github.list_installations` through the connection, refused first when it lacks
    /// [`ListInstallations::SCOPES`].
    pub async fn github_list_installations(
        &self,
        args: GithubActionArgs<'_, ListInstallations>,
    ) -> UseCaseResult<Vec<GithubInstallation>> {
        self.run("github_list_installations", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_installations",
                required_scopes: ListInstallations::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_installations(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
