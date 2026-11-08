//! `github.search_commits`: commits whose message and details match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{search::SearchCommits, types::GithubCommitSearchResults};

impl UseCases {
    /// Runs `github.search_commits` through the connection, refused first when it lacks
    /// [`SearchCommits::SCOPES`].
    pub async fn github_search_commits(
        &self,
        args: GithubActionArgs<'_, SearchCommits>,
    ) -> UseCaseResult<GithubCommitSearchResults> {
        self.run("github_search_commits", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_commits",
                required_scopes: SearchCommits::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .search_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
