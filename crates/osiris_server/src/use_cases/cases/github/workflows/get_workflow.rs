//! `github.get_workflow`: a workflow, by id or file name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubWorkflow, workflows::GetWorkflow};

impl UseCases {
    /// Runs `github.get_workflow` through the connection, refused first when it lacks
    /// [`GetWorkflow::SCOPES`].
    pub async fn github_get_workflow(
        &self,
        args: GithubActionArgs<'_, GetWorkflow>,
    ) -> UseCaseResult<GithubWorkflow> {
        self.run("github_get_workflow", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_workflow",
                required_scopes: GetWorkflow::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
