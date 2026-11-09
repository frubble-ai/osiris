//! `github.star_repository`: a repository starred by the authenticated user.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::StarRepository, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.star_repository` through the connection, refused first when it lacks
    /// [`StarRepository::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_star_repository(
        &self,
        args: ActionArgs<'_, StarRepository>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_star_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.star_repository",
                required_scopes: StarRepository::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .star_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
