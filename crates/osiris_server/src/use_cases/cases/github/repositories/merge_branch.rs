//! `github.merge_branch`: one branch merged into another.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::MergeBranch, types::GithubMergeResult};

impl UseCases {
    /// Runs `github.merge_branch` through the connection, refused first when it lacks
    /// [`MergeBranch::SCOPES`].
    pub async fn github_merge_branch(
        &self,
        args: GithubActionArgs<'_, MergeBranch>,
    ) -> UseCaseResult<GithubMergeResult> {
        self.run("github_merge_branch", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.merge_branch",
                required_scopes: MergeBranch::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .merge_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
