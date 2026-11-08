//! `github.list_repository_workflows`: the workflows a repository has, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubWorkflow, workflows::ListRepositoryWorkflows};

impl UseCases {
    /// Runs `github.list_repository_workflows` through the connection, refused first when it lacks
    /// [`ListRepositoryWorkflows::SCOPES`].
    pub async fn github_list_repository_workflows(
        &self,
        args: GithubActionArgs<'_, ListRepositoryWorkflows>,
    ) -> UseCaseResult<Vec<GithubWorkflow>> {
        self.run("github_list_repository_workflows", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_workflows",
                required_scopes: ListRepositoryWorkflows::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_workflows(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
