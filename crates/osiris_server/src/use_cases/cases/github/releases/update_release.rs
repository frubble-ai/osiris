//! `github.update_release`: a release changed, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::UpdateRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.update_release` through the connection, refused first when it lacks
    /// [`UpdateRelease::SCOPES`].
    pub async fn github_update_release(
        &self,
        args: ActionArgs<'_, UpdateRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_update_release", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_release",
                required_scopes: UpdateRelease::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
