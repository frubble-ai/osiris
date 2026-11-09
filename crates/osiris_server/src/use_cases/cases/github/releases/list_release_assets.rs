//! `github.list_release_assets`: the files attached to a release, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::ListReleaseAssets, types::GithubReleaseAsset};

impl UseCases {
    /// Runs `github.list_release_assets` through the connection, refused first when it lacks
    /// [`ListReleaseAssets::SCOPES`].
    pub async fn github_list_release_assets(
        &self,
        args: ActionArgs<'_, ListReleaseAssets>,
    ) -> UseCaseResult<Vec<GithubReleaseAsset>> {
        self.run("github_list_release_assets", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_release_assets",
                required_scopes: ListReleaseAssets::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_release_assets(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
