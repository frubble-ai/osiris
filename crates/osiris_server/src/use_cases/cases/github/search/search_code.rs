//! `github.search_code`: files whose code matches a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{search::SearchCode, types::GithubCodeSearchResults};

impl UseCases {
    /// Runs `github.search_code` through the connection, refused first when it lacks
    /// [`SearchCode::SCOPES`].
    pub async fn github_search_code(
        &self,
        args: GithubActionArgs<'_, SearchCode>,
    ) -> UseCaseResult<GithubCodeSearchResults> {
        self.run("github_search_code", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_code",
                required_scopes: SearchCode::SCOPES,
                call: move |token: Secret| async move {
                    github.search_code(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
