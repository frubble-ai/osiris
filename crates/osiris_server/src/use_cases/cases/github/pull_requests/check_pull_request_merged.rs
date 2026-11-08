//! `github.check_pull_request_merged`: whether a pull request has been merged.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::CheckPullRequestMerged, types::GithubMergedCheck};

impl UseCases {
    /// Runs `github.check_pull_request_merged` through the connection, refused first when it lacks
    /// [`CheckPullRequestMerged::SCOPES`].
    pub async fn github_check_pull_request_merged(
        &self,
        args: GithubActionArgs<'_, CheckPullRequestMerged>,
    ) -> UseCaseResult<GithubMergedCheck> {
        self.run("github_check_pull_request_merged", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.check_pull_request_merged",
                required_scopes: CheckPullRequestMerged::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .check_pull_request_merged(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
