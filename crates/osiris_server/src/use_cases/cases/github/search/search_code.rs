//! `github.search_code`: files whose code matches a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, search::SearchCode, types::GithubCodeSearchResults};

impl UseCases {
    /// Runs `github.search_code` through the connection, refused first when it lacks
    /// [`SearchCode::SCOPES`].
    pub async fn github_search_code(
        &self,
        args: ActionArgs<'_, SearchCode>,
    ) -> UseCaseResult<GithubCodeSearchResults> {
        self.run("github_search_code", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_code",
                required_scopes: SearchCode::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_code(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
