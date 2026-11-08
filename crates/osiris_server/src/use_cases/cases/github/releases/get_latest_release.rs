//! `github.get_latest_release`: a repository's latest published release.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::GetLatestRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.get_latest_release` through the connection, refused first when it lacks
    /// [`GetLatestRelease::SCOPES`].
    pub async fn github_get_latest_release(
        &self,
        args: ActionArgs<'_, GetLatestRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_get_latest_release", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_latest_release",
                required_scopes: GetLatestRelease::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_latest_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
