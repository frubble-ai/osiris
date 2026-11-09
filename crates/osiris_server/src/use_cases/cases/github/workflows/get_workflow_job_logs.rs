//! `github.get_workflow_job_logs`: the plain-text logs of a finished workflow job.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubJobLogs, workflows::GetWorkflowJobLogs};

impl UseCases {
    /// Runs `github.get_workflow_job_logs` through the connection, refused first when it lacks
    /// [`GetWorkflowJobLogs::SCOPES`].
    pub async fn github_get_workflow_job_logs(
        &self,
        args: ActionArgs<'_, GetWorkflowJobLogs>,
    ) -> UseCaseResult<GithubJobLogs> {
        self.run("github_get_workflow_job_logs", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_workflow_job_logs",
                required_scopes: GetWorkflowJobLogs::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_workflow_job_logs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
