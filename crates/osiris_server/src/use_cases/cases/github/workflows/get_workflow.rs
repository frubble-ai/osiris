//! `github.get_workflow`: a workflow, by id or file name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubWorkflow, workflows::GetWorkflow};

impl UseCases {
    /// Runs `github.get_workflow` through the connection, refused first when it lacks
    /// [`GetWorkflow::SCOPES`].
    pub async fn github_get_workflow(
        &self,
        args: ActionArgs<'_, GetWorkflow>,
    ) -> UseCaseResult<GithubWorkflow> {
        self.run("github_get_workflow", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_workflow",
                required_scopes: GetWorkflow::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_workflow(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
