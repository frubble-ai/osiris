//! `github.search_repositories`: repositories that match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{search::SearchRepositories, types::GithubRepositorySearchResults};

impl UseCases {
    /// Runs `github.search_repositories` through the connection, refused first when it lacks
    /// [`SearchRepositories::SCOPES`].
    pub async fn github_search_repositories(
        &self,
        args: GithubActionArgs<'_, SearchRepositories>,
    ) -> UseCaseResult<GithubRepositorySearchResults> {
        self.run("github_search_repositories", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_repositories",
                required_scopes: SearchRepositories::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .search_repositories(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
