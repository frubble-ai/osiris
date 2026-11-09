//! `github.list_my_starred_repositories`: the repositories the authenticated user starred, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, activity::ListMyStarredRepositories, types::GithubRepository,
};

impl UseCases {
    /// Runs `github.list_my_starred_repositories` through the connection, refused first when it lacks
    /// [`ListMyStarredRepositories::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_list_my_starred_repositories(
        &self,
        args: ActionArgs<'_, ListMyStarredRepositories>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_my_starred_repositories", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_my_starred_repositories",
                required_scopes: ListMyStarredRepositories::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_my_starred_repositories(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
