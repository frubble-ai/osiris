//! `github.cancel_workflow_run`: stops a workflow run that is queued or in progress.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubAcknowledgement, workflows::CancelWorkflowRun};

impl UseCases {
    /// Runs `github.cancel_workflow_run` through the connection, refused first when it lacks
    /// [`CancelWorkflowRun::SCOPES`].
    pub async fn github_cancel_workflow_run(
        &self,
        args: GithubActionArgs<'_, CancelWorkflowRun>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_cancel_workflow_run", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.cancel_workflow_run",
                required_scopes: CancelWorkflowRun::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .cancel_workflow_run(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
