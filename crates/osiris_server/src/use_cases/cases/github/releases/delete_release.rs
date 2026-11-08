//! `github.delete_release`: a release deleted, by its id. Its tag stays.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::DeleteRelease, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_release` through the connection, refused first when it lacks
    /// [`DeleteRelease::SCOPES`].
    pub async fn github_delete_release(
        &self,
        args: ActionArgs<'_, DeleteRelease>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_release", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_release",
                required_scopes: DeleteRelease::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
