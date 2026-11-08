//! `github.list_my_starred_repositories`: the repositories the authenticated user starred, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListMyStarredRepositories, types::GithubRepository};

impl UseCases {
    /// Runs `github.list_my_starred_repositories` through the connection, refused first when it lacks
    /// [`ListMyStarredRepositories::SCOPES`].
    pub async fn github_list_my_starred_repositories(
        &self,
        args: GithubActionArgs<'_, ListMyStarredRepositories>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_my_starred_repositories", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_my_starred_repositories",
                required_scopes: ListMyStarredRepositories::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_my_starred_repositories(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
