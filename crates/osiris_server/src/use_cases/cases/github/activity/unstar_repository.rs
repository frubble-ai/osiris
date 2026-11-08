//! `github.unstar_repository`: a repository's star taken back by the authenticated user.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::UnstarRepository, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.unstar_repository` through the connection, refused first when it lacks
    /// [`UnstarRepository::SCOPES`].
    pub async fn github_unstar_repository(
        &self,
        args: GithubActionArgs<'_, UnstarRepository>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_unstar_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.unstar_repository",
                required_scopes: UnstarRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .unstar_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
