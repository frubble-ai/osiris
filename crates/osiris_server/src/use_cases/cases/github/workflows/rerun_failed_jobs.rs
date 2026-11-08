//! `github.rerun_failed_jobs`: runs again the jobs of a workflow run that failed, and the jobs that depend on them.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubAcknowledgement, workflows::RerunFailedJobs};

impl UseCases {
    /// Runs `github.rerun_failed_jobs` through the connection, refused first when it lacks
    /// [`RerunFailedJobs::SCOPES`].
    pub async fn github_rerun_failed_jobs(
        &self,
        args: GithubActionArgs<'_, RerunFailedJobs>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerun_failed_jobs", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerun_failed_jobs",
                required_scopes: RerunFailedJobs::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .rerun_failed_jobs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
