//! `github.list_workflow_runs`: a repository's workflow runs, newest first, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubWorkflowRun, workflows::ListWorkflowRuns};

impl UseCases {
    /// Runs `github.list_workflow_runs` through the connection, refused first when it lacks
    /// [`ListWorkflowRuns::SCOPES`].
    pub async fn github_list_workflow_runs(
        &self,
        args: GithubActionArgs<'_, ListWorkflowRuns>,
    ) -> UseCaseResult<Vec<GithubWorkflowRun>> {
        self.run("github_list_workflow_runs", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_workflow_runs",
                required_scopes: ListWorkflowRuns::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_workflow_runs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
