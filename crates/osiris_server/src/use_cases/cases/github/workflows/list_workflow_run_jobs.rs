//! `github.list_workflow_run_jobs`: the jobs of a workflow run, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubWorkflowJob, workflows::ListWorkflowRunJobs};

impl UseCases {
    /// Runs `github.list_workflow_run_jobs` through the connection, refused first when it lacks
    /// [`ListWorkflowRunJobs::SCOPES`].
    pub async fn github_list_workflow_run_jobs(
        &self,
        args: ActionArgs<'_, ListWorkflowRunJobs>,
    ) -> UseCaseResult<Vec<GithubWorkflowJob>> {
        self.run("github_list_workflow_run_jobs", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_workflow_run_jobs",
                required_scopes: ListWorkflowRunJobs::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_workflow_run_jobs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
