//! `github.disable_workflow`: turns a workflow off, so nothing starts it until it is enabled again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubAcknowledgement, workflows::DisableWorkflow};

impl UseCases {
    /// Runs `github.disable_workflow` through the connection, refused first when it lacks
    /// [`DisableWorkflow::SCOPES`].
    pub async fn github_disable_workflow(
        &self,
        args: ActionArgs<'_, DisableWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_disable_workflow", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.disable_workflow",
                required_scopes: DisableWorkflow::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .disable_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
