//! `github.delete_ref`: a branch's or a tag's git reference deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::DeleteRef, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_ref` through the connection, refused first when it lacks
    /// [`DeleteRef::SCOPES`].
    pub async fn github_delete_ref(
        &self,
        args: ActionArgs<'_, DeleteRef>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_ref", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_ref",
                required_scopes: DeleteRef::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_ref(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
