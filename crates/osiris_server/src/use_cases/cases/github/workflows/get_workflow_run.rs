//! `github.get_workflow_run`: a workflow run, by id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubWorkflowRun, workflows::GetWorkflowRun};

impl UseCases {
    /// Runs `github.get_workflow_run` through the connection, refused first when it lacks
    /// [`GetWorkflowRun::SCOPES`].
    pub async fn github_get_workflow_run(
        &self,
        args: ActionArgs<'_, GetWorkflowRun>,
    ) -> UseCaseResult<GithubWorkflowRun> {
        self.run("github_get_workflow_run", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_workflow_run",
                required_scopes: GetWorkflowRun::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_workflow_run(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
