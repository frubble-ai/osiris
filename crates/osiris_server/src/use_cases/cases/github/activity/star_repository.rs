//! `github.star_repository`: a repository starred by the authenticated user.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::StarRepository, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.star_repository` through the connection, refused first when it lacks
    /// [`StarRepository::SCOPES`].
    pub async fn github_star_repository(
        &self,
        args: GithubActionArgs<'_, StarRepository>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_star_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.star_repository",
                required_scopes: StarRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .star_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
