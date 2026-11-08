//! `github.rerequest_check_suite`: asks the app that made a check suite to run it again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{checks::RerequestCheckSuite, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.rerequest_check_suite` through the connection, refused first when it lacks
    /// [`RerequestCheckSuite::SCOPES`].
    pub async fn github_rerequest_check_suite(
        &self,
        args: GithubActionArgs<'_, RerequestCheckSuite>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerequest_check_suite", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerequest_check_suite",
                required_scopes: RerequestCheckSuite::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .rerequest_check_suite(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
