//! `github.search_users`: users that match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{search::SearchUsers, types::GithubUserSearchResults};

impl UseCases {
    /// Runs `github.search_users` through the connection, refused first when it lacks
    /// [`SearchUsers::SCOPES`].
    pub async fn github_search_users(
        &self,
        args: GithubActionArgs<'_, SearchUsers>,
    ) -> UseCaseResult<GithubUserSearchResults> {
        self.run("github_search_users", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_users",
                required_scopes: SearchUsers::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .search_users(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
