//! `github.list_workflow_run_artifacts`: the artifacts a workflow run kept, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubArtifact, workflows::ListWorkflowRunArtifacts};

impl UseCases {
    /// Runs `github.list_workflow_run_artifacts` through the connection, refused first when it lacks
    /// [`ListWorkflowRunArtifacts::SCOPES`].
    pub async fn github_list_workflow_run_artifacts(
        &self,
        args: GithubActionArgs<'_, ListWorkflowRunArtifacts>,
    ) -> UseCaseResult<Vec<GithubArtifact>> {
        self.run("github_list_workflow_run_artifacts", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_workflow_run_artifacts",
                required_scopes: ListWorkflowRunArtifacts::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_workflow_run_artifacts(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
