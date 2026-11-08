//! `github.request_pull_request_reviewers`: a pull request's review requested from users and teams.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::RequestPullRequestReviewers, types::GithubReviewRequest,
};

impl UseCases {
    /// Runs `github.request_pull_request_reviewers` through the connection, refused first when it lacks
    /// [`RequestPullRequestReviewers::SCOPES`].
    pub async fn github_request_pull_request_reviewers(
        &self,
        args: GithubActionArgs<'_, RequestPullRequestReviewers>,
    ) -> UseCaseResult<GithubReviewRequest> {
        self.run("github_request_pull_request_reviewers", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.request_pull_request_reviewers",
                required_scopes: RequestPullRequestReviewers::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .request_pull_request_reviewers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
