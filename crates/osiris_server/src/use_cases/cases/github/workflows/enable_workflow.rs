//! `github.enable_workflow`: turns a disabled workflow back on.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubAcknowledgement, workflows::EnableWorkflow};

impl UseCases {
    /// Runs `github.enable_workflow` through the connection, refused first when it lacks
    /// [`EnableWorkflow::SCOPES`].
    pub async fn github_enable_workflow(
        &self,
        args: GithubActionArgs<'_, EnableWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_enable_workflow", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.enable_workflow",
                required_scopes: EnableWorkflow::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .enable_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
