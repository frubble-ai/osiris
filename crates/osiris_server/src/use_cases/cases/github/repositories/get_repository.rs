//! `github.get_repository`: a repository, by owner and name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::GetRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.get_repository` through the connection, refused first when it lacks
    /// [`GetRepository::SCOPES`].
    pub async fn github_get_repository(
        &self,
        args: ActionArgs<'_, GetRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_get_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_repository",
                required_scopes: GetRepository::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
