//! `github.get_release`: a release, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::GetRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.get_release` through the connection, refused first when it lacks
    /// [`GetRelease::SCOPES`].
    pub async fn github_get_release(
        &self,
        args: ActionArgs<'_, GetRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_get_release", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_release",
                required_scopes: GetRelease::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
