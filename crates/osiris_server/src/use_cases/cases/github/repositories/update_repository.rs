//! `github.update_repository`: a repository's settings and metadata changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::UpdateRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.update_repository` through the connection, refused first when it lacks
    /// [`UpdateRepository::SCOPES`].
    pub async fn github_update_repository(
        &self,
        args: GithubActionArgs<'_, UpdateRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_update_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_repository",
                required_scopes: UpdateRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
