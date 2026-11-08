//! `github.get_workflow_job_logs`: the plain-text logs of a finished workflow job.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubJobLogs, workflows::GetWorkflowJobLogs};

impl UseCases {
    /// Runs `github.get_workflow_job_logs` through the connection, refused first when it lacks
    /// [`GetWorkflowJobLogs::SCOPES`].
    pub async fn github_get_workflow_job_logs(
        &self,
        args: GithubActionArgs<'_, GetWorkflowJobLogs>,
    ) -> UseCaseResult<GithubJobLogs> {
        self.run("github_get_workflow_job_logs", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_workflow_job_logs",
                required_scopes: GetWorkflowJobLogs::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_workflow_job_logs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
