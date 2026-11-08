//! `github.check_repository_starred`: whether the authenticated user has starred a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::CheckRepositoryStarred, types::GithubStarredCheck};

impl UseCases {
    /// Runs `github.check_repository_starred` through the connection, refused first when it lacks
    /// [`CheckRepositoryStarred::SCOPES`].
    pub async fn github_check_repository_starred(
        &self,
        args: GithubActionArgs<'_, CheckRepositoryStarred>,
    ) -> UseCaseResult<GithubStarredCheck> {
        self.run("github_check_repository_starred", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.check_repository_starred",
                required_scopes: CheckRepositoryStarred::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .check_repository_starred(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
