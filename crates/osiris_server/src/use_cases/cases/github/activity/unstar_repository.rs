//! `github.unstar_repository`: a repository's star taken back by the authenticated user.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::UnstarRepository, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.unstar_repository` through the connection, refused first when it lacks
    /// [`UnstarRepository::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_unstar_repository(
        &self,
        args: ActionArgs<'_, UnstarRepository>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_unstar_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.unstar_repository",
                required_scopes: UnstarRepository::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .unstar_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
