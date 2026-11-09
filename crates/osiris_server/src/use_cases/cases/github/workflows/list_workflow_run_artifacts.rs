//! `github.list_workflow_run_artifacts`: the artifacts a workflow run kept, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, types::GithubArtifact, workflows::ListWorkflowRunArtifacts,
};

impl UseCases {
    /// Runs `github.list_workflow_run_artifacts` through the connection, refused first when it lacks
    /// [`ListWorkflowRunArtifacts::SCOPES`].
    pub async fn github_list_workflow_run_artifacts(
        &self,
        args: ActionArgs<'_, ListWorkflowRunArtifacts>,
    ) -> UseCaseResult<Vec<GithubArtifact>> {
        self.run("github_list_workflow_run_artifacts", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_workflow_run_artifacts",
                required_scopes: ListWorkflowRunArtifacts::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_workflow_run_artifacts(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
