//! `github.fork_repository`: a fork of a repository, made in the background.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::ForkRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.fork_repository` through the connection, refused first when it lacks
    /// [`ForkRepository::SCOPES`].
    pub async fn github_fork_repository(
        &self,
        args: ActionArgs<'_, ForkRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_fork_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.fork_repository",
                required_scopes: ForkRepository::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .fork_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
