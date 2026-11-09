//! `github.check_repository_starred`: whether the authenticated user has starred a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, activity::CheckRepositoryStarred, types::GithubStarredCheck,
};

impl UseCases {
    /// Runs `github.check_repository_starred` through the connection, refused first when it lacks
    /// [`CheckRepositoryStarred::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_check_repository_starred(
        &self,
        args: ActionArgs<'_, CheckRepositoryStarred>,
    ) -> UseCaseResult<GithubStarredCheck> {
        self.run("github_check_repository_starred", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.check_repository_starred",
                required_scopes: CheckRepositoryStarred::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .check_repository_starred(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
