//! `github.list_my_repositories`: the repositories the authenticated user can see, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::ListMyRepositories, types::GithubRepository};

impl UseCases {
    /// Runs `github.list_my_repositories` through the connection, refused first when it lacks
    /// [`ListMyRepositories::SCOPES`].
    pub async fn github_list_my_repositories(
        &self,
        args: ActionArgs<'_, ListMyRepositories>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_my_repositories", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_my_repositories",
                required_scopes: ListMyRepositories::SCOPES,
                user_only: false,
                call: move |token: Secret, acting: ActingAs| async move {
                    provider
                        .list_my_repositories(http, token.expose_secret(), input, acting)
                        .await
                },
            })
            .await
        })
        .await
    }
}
