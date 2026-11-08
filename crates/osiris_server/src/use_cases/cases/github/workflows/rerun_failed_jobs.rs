//! `github.rerun_failed_jobs`: runs again the jobs of a workflow run that failed, and the jobs that depend on them.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubAcknowledgement, workflows::RerunFailedJobs};

impl UseCases {
    /// Runs `github.rerun_failed_jobs` through the connection, refused first when it lacks
    /// [`RerunFailedJobs::SCOPES`].
    pub async fn github_rerun_failed_jobs(
        &self,
        args: ActionArgs<'_, RerunFailedJobs>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerun_failed_jobs", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerun_failed_jobs",
                required_scopes: RerunFailedJobs::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .rerun_failed_jobs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
