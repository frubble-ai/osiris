//! `github.create_repository`: a new repository of the authenticated user's.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::CreateRepository, types::GithubRepository};

impl UseCases {
    /// Runs `github.create_repository` through the connection, refused first when it lacks
    /// [`CreateRepository::SCOPES`].
    pub async fn github_create_repository(
        &self,
        args: GithubActionArgs<'_, CreateRepository>,
    ) -> UseCaseResult<GithubRepository> {
        self.run("github_create_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_repository",
                required_scopes: CreateRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
