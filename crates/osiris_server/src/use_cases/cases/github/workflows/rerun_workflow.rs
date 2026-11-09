//! `github.rerun_workflow`: runs every job of a workflow run again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubAcknowledgement, workflows::RerunWorkflow};

impl UseCases {
    /// Runs `github.rerun_workflow` through the connection, refused first when it lacks
    /// [`RerunWorkflow::SCOPES`].
    pub async fn github_rerun_workflow(
        &self,
        args: ActionArgs<'_, RerunWorkflow>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_rerun_workflow", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rerun_workflow",
                required_scopes: RerunWorkflow::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .rerun_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
