//! `github.get_workflow_run`: a workflow run, by id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubWorkflowRun, workflows::GetWorkflowRun};

impl UseCases {
    /// Runs `github.get_workflow_run` through the connection, refused first when it lacks
    /// [`GetWorkflowRun::SCOPES`].
    pub async fn github_get_workflow_run(
        &self,
        args: GithubActionArgs<'_, GetWorkflowRun>,
    ) -> UseCaseResult<GithubWorkflowRun> {
        self.run("github_get_workflow_run", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_workflow_run",
                required_scopes: GetWorkflowRun::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_workflow_run(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
