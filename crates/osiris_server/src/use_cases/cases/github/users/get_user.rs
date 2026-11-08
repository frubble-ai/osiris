//! `github.get_user`: a user's public profile, by login.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubPublicUser, users::GetUser};

impl UseCases {
    /// Runs `github.get_user` through the connection, refused first when it lacks
    /// [`GetUser::SCOPES`].
    pub async fn github_get_user(
        &self,
        args: ActionArgs<'_, GetUser>,
    ) -> UseCaseResult<GithubPublicUser> {
        self.run("github_get_user", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_user",
                required_scopes: GetUser::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider.get_user(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
