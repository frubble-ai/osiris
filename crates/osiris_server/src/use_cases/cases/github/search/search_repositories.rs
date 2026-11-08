//! `github.search_repositories`: repositories that match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, search::SearchRepositories, types::GithubRepositorySearchResults,
};

impl UseCases {
    /// Runs `github.search_repositories` through the connection, refused first when it lacks
    /// [`SearchRepositories::SCOPES`].
    pub async fn github_search_repositories(
        &self,
        args: ActionArgs<'_, SearchRepositories>,
    ) -> UseCaseResult<GithubRepositorySearchResults> {
        self.run("github_search_repositories", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_repositories",
                required_scopes: SearchRepositories::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_repositories(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
