//! `github.list_user_repositories`: a user's public repositories, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::ListUserRepositories, types::GithubRepository,
};

impl UseCases {
    /// Runs `github.list_user_repositories` through the connection, refused first when it lacks
    /// [`ListUserRepositories::SCOPES`].
    pub async fn github_list_user_repositories(
        &self,
        args: ActionArgs<'_, ListUserRepositories>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_user_repositories", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_user_repositories",
                required_scopes: ListUserRepositories::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_user_repositories(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
