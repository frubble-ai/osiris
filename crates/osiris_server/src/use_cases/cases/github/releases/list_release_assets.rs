//! `github.list_release_assets`: the files attached to a release, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::ListReleaseAssets, types::GithubReleaseAsset};

impl UseCases {
    /// Runs `github.list_release_assets` through the connection, refused first when it lacks
    /// [`ListReleaseAssets::SCOPES`].
    pub async fn github_list_release_assets(
        &self,
        args: GithubActionArgs<'_, ListReleaseAssets>,
    ) -> UseCaseResult<Vec<GithubReleaseAsset>> {
        self.run("github_list_release_assets", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_release_assets",
                required_scopes: ListReleaseAssets::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_release_assets(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
