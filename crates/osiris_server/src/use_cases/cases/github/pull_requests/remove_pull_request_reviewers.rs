//! `github.remove_pull_request_reviewers`: users and teams removed from those a pull request's review is requested from.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::RemovePullRequestReviewers, types::GithubReviewRequest,
};

impl UseCases {
    /// Runs `github.remove_pull_request_reviewers` through the connection, refused first when it lacks
    /// [`RemovePullRequestReviewers::SCOPES`].
    pub async fn github_remove_pull_request_reviewers(
        &self,
        args: GithubActionArgs<'_, RemovePullRequestReviewers>,
    ) -> UseCaseResult<GithubReviewRequest> {
        self.run("github_remove_pull_request_reviewers", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_pull_request_reviewers",
                required_scopes: RemovePullRequestReviewers::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .remove_pull_request_reviewers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
