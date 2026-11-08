//! `github.dispatch_workflow`: starts a workflow that runs on `workflow_dispatch`.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubAcknowledgement, workflows::DispatchWorkflow};

impl UseCases {
    /// Runs `github.dispatch_workflow` through the connection, refused first when it lacks
    /// [`DispatchWorkflow::SCOPES`].
    pub async fn github_dispatch_workflow(
        &self,
        args: GithubActionArgs<'_, DispatchWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_dispatch_workflow", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.dispatch_workflow",
                required_scopes: DispatchWorkflow::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .dispatch_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
