//! `github.delete_release_asset`: a file attached to a release deleted, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, releases::DeleteReleaseAsset, types::GithubAcknowledgement,
};

impl UseCases {
    /// Runs `github.delete_release_asset` through the connection, refused first when it lacks
    /// [`DeleteReleaseAsset::SCOPES`].
    pub async fn github_delete_release_asset(
        &self,
        args: ActionArgs<'_, DeleteReleaseAsset>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_release_asset", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_release_asset",
                required_scopes: DeleteReleaseAsset::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_release_asset(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
