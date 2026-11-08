//! `github.merge_pull_request`: a pull request merged into its base branch.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::MergePullRequest, types::GithubMergeResult};

impl UseCases {
    /// Runs `github.merge_pull_request` through the connection, refused first when it lacks
    /// [`MergePullRequest::SCOPES`].
    pub async fn github_merge_pull_request(
        &self,
        args: GithubActionArgs<'_, MergePullRequest>,
    ) -> UseCaseResult<GithubMergeResult> {
        self.run("github_merge_pull_request", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.merge_pull_request",
                required_scopes: MergePullRequest::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .merge_pull_request(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
