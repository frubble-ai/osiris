//! `github.rename_branch`: a branch given a new name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::RenameBranch, types::GithubBranch};

impl UseCases {
    /// Runs `github.rename_branch` through the connection, refused first when it lacks
    /// [`RenameBranch::SCOPES`].
    pub async fn github_rename_branch(
        &self,
        args: GithubActionArgs<'_, RenameBranch>,
    ) -> UseCaseResult<GithubBranch> {
        self.run("github_rename_branch", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rename_branch",
                required_scopes: RenameBranch::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .rename_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
