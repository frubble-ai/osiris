//! `github.list_repositories`: every repository the GitHub App's installations give the connected user, each once.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{installations::ListRepositories, types::GithubRepository};

impl UseCases {
    /// Runs `github.list_repositories` through the connection, refused first when it lacks
    /// [`ListRepositories::SCOPES`].
    pub async fn github_list_repositories(
        &self,
        args: GithubActionArgs<'_, ListRepositories>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_repositories", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repositories",
                required_scopes: ListRepositories::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repositories(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
