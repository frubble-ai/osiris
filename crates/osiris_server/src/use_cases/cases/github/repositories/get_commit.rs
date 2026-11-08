//! `github.get_commit`: a commit, with its stats and the files it changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::GetCommit, types::GithubCommit};

impl UseCases {
    /// Runs `github.get_commit` through the connection, refused first when it lacks
    /// [`GetCommit::SCOPES`].
    pub async fn github_get_commit(
        &self,
        args: GithubActionArgs<'_, GetCommit>,
    ) -> UseCaseResult<GithubCommit> {
        self.run("github_get_commit", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_commit",
                required_scopes: GetCommit::SCOPES,
                call: move |token: Secret| async move {
                    github.get_commit(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
