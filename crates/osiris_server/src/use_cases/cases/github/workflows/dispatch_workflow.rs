//! `github.dispatch_workflow`: starts a workflow that runs on `workflow_dispatch`.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubAcknowledgement, workflows::DispatchWorkflow};

impl UseCases {
    /// Runs `github.dispatch_workflow` through the connection, refused first when it lacks
    /// [`DispatchWorkflow::SCOPES`].
    pub async fn github_dispatch_workflow(
        &self,
        args: ActionArgs<'_, DispatchWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_dispatch_workflow", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.dispatch_workflow",
                required_scopes: DispatchWorkflow::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .dispatch_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
