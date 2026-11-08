//! `github.rerequest_check_run`: asks the app that made a check run to run it again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{checks::RerequestCheckRun, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.rerequest_check_run` through the connection, refused first when it lacks
    /// [`RerequestCheckRun::SCOPES`].
    pub async fn github_rerequest_check_run(
        &self,
        args: GithubActionArgs<'_, RerequestCheckRun>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerequest_check_run", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerequest_check_run",
                required_scopes: RerequestCheckRun::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .rerequest_check_run(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
