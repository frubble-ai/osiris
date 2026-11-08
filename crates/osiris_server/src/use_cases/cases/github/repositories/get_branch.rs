//! `github.get_branch`: a branch, by name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::GetBranch, types::GithubBranch};

impl UseCases {
    /// Runs `github.get_branch` through the connection, refused first when it lacks
    /// [`GetBranch::SCOPES`].
    pub async fn github_get_branch(
        &self,
        args: GithubActionArgs<'_, GetBranch>,
    ) -> UseCaseResult<GithubBranch> {
        self.run("github_get_branch", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_branch",
                required_scopes: GetBranch::SCOPES,
                call: move |token: Secret| async move {
                    github.get_branch(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
