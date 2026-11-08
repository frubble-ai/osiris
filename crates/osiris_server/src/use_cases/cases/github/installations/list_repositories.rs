//! `github.list_repositories`: every repository the GitHub App's installations give the connected user, each once.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, installations::ListRepositories, types::GithubRepository};

impl UseCases {
    /// Runs `github.list_repositories` through the connection, refused first when it lacks
    /// [`ListRepositories::SCOPES`].
    pub async fn github_list_repositories(
        &self,
        args: ActionArgs<'_, ListRepositories>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_repositories", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repositories",
                required_scopes: ListRepositories::SCOPES,
                user_only: false,
                call: move |token: Secret, acting: ActingAs| async move {
                    provider
                        .list_repositories(http, token.expose_secret(), input, acting)
                        .await
                },
            })
            .await
        })
        .await
    }
}
