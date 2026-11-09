//! `github.update_repository`: a repository's settings and metadata changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::UpdateRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.update_repository` through the connection, refused first when it lacks
    /// [`UpdateRepository::SCOPES`].
    pub async fn github_update_repository(
        &self,
        args: ActionArgs<'_, UpdateRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_update_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_repository",
                required_scopes: UpdateRepository::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
