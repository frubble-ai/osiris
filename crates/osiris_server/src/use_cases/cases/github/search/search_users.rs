//! `github.search_users`: users that match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, search::SearchUsers, types::GithubUserSearchResults};

impl UseCases {
    /// Runs `github.search_users` through the connection, refused first when it lacks
    /// [`SearchUsers::SCOPES`].
    pub async fn github_search_users(
        &self,
        args: ActionArgs<'_, SearchUsers>,
    ) -> UseCaseResult<GithubUserSearchResults> {
        self.run("github_search_users", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_users",
                required_scopes: SearchUsers::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_users(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
