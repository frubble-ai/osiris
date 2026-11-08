//! `github.create_pull_request_review`: a review of a pull request, with comments on lines of its diff if given.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::CreatePullRequestReview, types::GithubReview};

impl UseCases {
    /// Runs `github.create_pull_request_review` through the connection, refused first when it lacks
    /// [`CreatePullRequestReview::SCOPES`].
    pub async fn github_create_pull_request_review(
        &self,
        args: GithubActionArgs<'_, CreatePullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_create_pull_request_review", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_pull_request_review",
                required_scopes: CreatePullRequestReview::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
