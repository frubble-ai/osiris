//! `github.enable_workflow`: turns a disabled workflow back on.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubAcknowledgement, workflows::EnableWorkflow};

impl UseCases {
    /// Runs `github.enable_workflow` through the connection, refused first when it lacks
    /// [`EnableWorkflow::SCOPES`].
    pub async fn github_enable_workflow(
        &self,
        args: ActionArgs<'_, EnableWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_enable_workflow", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.enable_workflow",
                required_scopes: EnableWorkflow::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .enable_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
