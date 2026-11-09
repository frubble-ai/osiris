//! `github.get_current_user`: the authenticated user and their profile.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubCurrentUser, users::GetCurrentUser};

impl UseCases {
    /// Runs `github.get_current_user` through the connection, refused first when it lacks
    /// [`GetCurrentUser::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_get_current_user(
        &self,
        args: ActionArgs<'_, GetCurrentUser>,
    ) -> UseCaseResult<GithubCurrentUser> {
        self.run("github_get_current_user", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_current_user",
                required_scopes: GetCurrentUser::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_current_user(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
