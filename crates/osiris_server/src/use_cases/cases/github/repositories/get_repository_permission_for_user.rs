//! `github.get_repository_permission_for_user`: what an account may do in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    repositories::GetRepositoryPermissionForUser, types::GithubRepositoryPermission,
};

impl UseCases {
    /// Runs `github.get_repository_permission_for_user` through the connection, refused first when it lacks
    /// [`GetRepositoryPermissionForUser::SCOPES`].
    pub async fn github_get_repository_permission_for_user(
        &self,
        args: GithubActionArgs<'_, GetRepositoryPermissionForUser>,
    ) -> UseCaseResult<GithubRepositoryPermission> {
        self.run("github_get_repository_permission_for_user", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_repository_permission_for_user",
                required_scopes: GetRepositoryPermissionForUser::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_repository_permission_for_user(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
