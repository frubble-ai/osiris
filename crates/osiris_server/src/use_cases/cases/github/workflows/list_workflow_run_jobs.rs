//! `github.list_workflow_run_jobs`: the jobs of a workflow run, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubWorkflowJob, workflows::ListWorkflowRunJobs};

impl UseCases {
    /// Runs `github.list_workflow_run_jobs` through the connection, refused first when it lacks
    /// [`ListWorkflowRunJobs::SCOPES`].
    pub async fn github_list_workflow_run_jobs(
        &self,
        args: GithubActionArgs<'_, ListWorkflowRunJobs>,
    ) -> UseCaseResult<Vec<GithubWorkflowJob>> {
        self.run("github_list_workflow_run_jobs", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_workflow_run_jobs",
                required_scopes: ListWorkflowRunJobs::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_workflow_run_jobs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
