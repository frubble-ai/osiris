//! `github.list_workflow_runs`: a repository's workflow runs, newest first, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, types::GithubWorkflowRun, workflows::ListWorkflowRuns};

impl UseCases {
    /// Runs `github.list_workflow_runs` through the connection, refused first when it lacks
    /// [`ListWorkflowRuns::SCOPES`].
    pub async fn github_list_workflow_runs(
        &self,
        args: ActionArgs<'_, ListWorkflowRuns>,
    ) -> UseCaseResult<Vec<GithubWorkflowRun>> {
        self.run("github_list_workflow_runs", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_workflow_runs",
                required_scopes: ListWorkflowRuns::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_workflow_runs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
