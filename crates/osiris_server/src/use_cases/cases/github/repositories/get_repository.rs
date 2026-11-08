//! `github.get_repository`: a repository, by owner and name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::GetRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.get_repository` through the connection, refused first when it lacks
    /// [`GetRepository::SCOPES`].
    pub async fn github_get_repository(
        &self,
        args: GithubActionArgs<'_, GetRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_get_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_repository",
                required_scopes: GetRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
