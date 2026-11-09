//! `github.rerequest_check_suite`: asks the app that made a check suite to run it again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, checks::RerequestCheckSuite, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.rerequest_check_suite` through the connection, refused first when it lacks
    /// [`RerequestCheckSuite::SCOPES`].
    pub async fn github_rerequest_check_suite(
        &self,
        args: ActionArgs<'_, RerequestCheckSuite>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerequest_check_suite", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerequest_check_suite",
                required_scopes: RerequestCheckSuite::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .rerequest_check_suite(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
