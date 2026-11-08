//! `github.rerun_workflow`: runs every job of a workflow run again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubAcknowledgement, workflows::RerunWorkflow};

impl UseCases {
    /// Runs `github.rerun_workflow` through the connection, refused first when it lacks
    /// [`RerunWorkflow::SCOPES`].
    pub async fn github_rerun_workflow(
        &self,
        args: GithubActionArgs<'_, RerunWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerun_workflow", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerun_workflow",
                required_scopes: RerunWorkflow::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .rerun_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
