//! `github.get_release_asset`: a file attached to a release, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::GetReleaseAsset, types::GithubReleaseAsset};

impl UseCases {
    /// Runs `github.get_release_asset` through the connection, refused first when it lacks
    /// [`GetReleaseAsset::SCOPES`].
    pub async fn github_get_release_asset(
        &self,
        args: GithubActionArgs<'_, GetReleaseAsset>,
    ) -> UseCaseResult<GithubReleaseAsset> {
        self.run("github_get_release_asset", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_release_asset",
                required_scopes: GetReleaseAsset::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_release_asset(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
