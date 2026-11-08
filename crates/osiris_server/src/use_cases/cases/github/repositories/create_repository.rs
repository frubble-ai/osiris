//! `github.create_repository`: a new repository of the authenticated user's.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::CreateRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.create_repository` through the connection, refused first when it lacks
    /// [`CreateRepository::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_create_repository(
        &self,
        args: ActionArgs<'_, CreateRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_create_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_repository",
                required_scopes: CreateRepository::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
