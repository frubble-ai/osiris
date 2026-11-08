//! `github.rerequest_check_run`: asks the app that made a check run to run it again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, checks::RerequestCheckRun, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.rerequest_check_run` through the connection, refused first when it lacks
    /// [`RerequestCheckRun::SCOPES`].
    pub async fn github_rerequest_check_run(
        &self,
        args: ActionArgs<'_, RerequestCheckRun>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerequest_check_run", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerequest_check_run",
                required_scopes: RerequestCheckRun::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .rerequest_check_run(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
