//! `github.disable_workflow`: turns a workflow off, so nothing starts it until it is enabled again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubAcknowledgement, workflows::DisableWorkflow};

impl UseCases {
    /// Runs `github.disable_workflow` through the connection, refused first when it lacks
    /// [`DisableWorkflow::SCOPES`].
    pub async fn github_disable_workflow(
        &self,
        args: GithubActionArgs<'_, DisableWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_disable_workflow", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.disable_workflow",
                required_scopes: DisableWorkflow::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .disable_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
