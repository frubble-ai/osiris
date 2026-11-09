//! `github.delete_label`: a label deleted from a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::DeleteLabel, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_label` through the connection, refused first when it lacks
    /// [`DeleteLabel::SCOPES`].
    pub async fn github_delete_label(
        &self,
        args: ActionArgs<'_, DeleteLabel>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_label", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_label",
                required_scopes: DeleteLabel::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
