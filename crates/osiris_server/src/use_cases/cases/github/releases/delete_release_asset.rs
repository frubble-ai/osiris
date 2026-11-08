//! `github.delete_release_asset`: a file attached to a release deleted, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::DeleteReleaseAsset, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_release_asset` through the connection, refused first when it lacks
    /// [`DeleteReleaseAsset::SCOPES`].
    pub async fn github_delete_release_asset(
        &self,
        args: GithubActionArgs<'_, DeleteReleaseAsset>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_release_asset", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_release_asset",
                required_scopes: DeleteReleaseAsset::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_release_asset(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
