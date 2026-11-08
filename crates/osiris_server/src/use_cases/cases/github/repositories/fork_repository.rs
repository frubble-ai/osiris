//! `github.fork_repository`: a fork of a repository, made in the background.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ForkRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.fork_repository` through the connection, refused first when it lacks
    /// [`ForkRepository::SCOPES`].
    pub async fn github_fork_repository(
        &self,
        args: GithubActionArgs<'_, ForkRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_fork_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.fork_repository",
                required_scopes: ForkRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .fork_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
