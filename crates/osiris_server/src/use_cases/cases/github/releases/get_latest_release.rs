//! `github.get_latest_release`: a repository's latest published release.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::GetLatestRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.get_latest_release` through the connection, refused first when it lacks
    /// [`GetLatestRelease::SCOPES`].
    pub async fn github_get_latest_release(
        &self,
        args: GithubActionArgs<'_, GetLatestRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_get_latest_release", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_latest_release",
                required_scopes: GetLatestRelease::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_latest_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
