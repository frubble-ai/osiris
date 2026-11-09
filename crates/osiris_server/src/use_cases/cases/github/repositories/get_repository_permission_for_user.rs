//! `github.get_repository_permission_for_user`: what an account may do in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::GetRepositoryPermissionForUser, types::GithubRepositoryPermission,
};

impl UseCases {
    /// Runs `github.get_repository_permission_for_user` through the connection, refused first when it lacks
    /// [`GetRepositoryPermissionForUser::SCOPES`].
    pub async fn github_get_repository_permission_for_user(
        &self,
        args: ActionArgs<'_, GetRepositoryPermissionForUser>,
    ) -> UseCaseResult<GithubRepositoryPermission> {
        self.run("github_get_repository_permission_for_user", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_repository_permission_for_user",
                required_scopes: GetRepositoryPermissionForUser::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_repository_permission_for_user(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
