//! `github.search_commits`: commits whose message and details match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, search::SearchCommits, types::GithubCommitSearchResults};

impl UseCases {
    /// Runs `github.search_commits` through the connection, refused first when it lacks
    /// [`SearchCommits::SCOPES`].
    pub async fn github_search_commits(
        &self,
        args: ActionArgs<'_, SearchCommits>,
    ) -> UseCaseResult<GithubCommitSearchResults> {
        self.run("github_search_commits", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_commits",
                required_scopes: SearchCommits::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
