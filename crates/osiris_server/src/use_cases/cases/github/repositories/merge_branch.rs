//! `github.merge_branch`: one branch merged into another.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::MergeBranch, types::GithubMergeResult};

impl UseCases {
    /// Runs `github.merge_branch` through the connection, refused first when it lacks
    /// [`MergeBranch::SCOPES`].
    pub async fn github_merge_branch(
        &self,
        args: ActionArgs<'_, MergeBranch>,
    ) -> UseCaseResult<GithubMergeResult> {
        self.run("github_merge_branch", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.merge_branch",
                required_scopes: MergeBranch::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .merge_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
