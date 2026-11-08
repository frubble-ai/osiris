//! `github.update_pull_request_branch`: a pull request's branch brought up to date with the latest changes of its base branch.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::UpdatePullRequestBranch, types::GithubPullRequestBranchUpdate,
};

impl UseCases {
    /// Runs `github.update_pull_request_branch` through the connection, refused first when it lacks
    /// [`UpdatePullRequestBranch::SCOPES`].
    pub async fn github_update_pull_request_branch(
        &self,
        args: GithubActionArgs<'_, UpdatePullRequestBranch>,
    ) -> UseCaseResult<GithubPullRequestBranchUpdate> {
        self.run("github_update_pull_request_branch", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_pull_request_branch",
                required_scopes: UpdatePullRequestBranch::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_pull_request_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
