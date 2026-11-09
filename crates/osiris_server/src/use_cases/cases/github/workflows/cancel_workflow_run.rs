//! `github.cancel_workflow_run`: stops a workflow run that is queued or in progress.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, types::GithubAcknowledgement, workflows::CancelWorkflowRun,
};

impl UseCases {
    /// Runs `github.cancel_workflow_run` through the connection, refused first when it lacks
    /// [`CancelWorkflowRun::SCOPES`].
    pub async fn github_cancel_workflow_run(
        &self,
        args: ActionArgs<'_, CancelWorkflowRun>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_cancel_workflow_run", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.cancel_workflow_run",
                required_scopes: CancelWorkflowRun::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .cancel_workflow_run(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
