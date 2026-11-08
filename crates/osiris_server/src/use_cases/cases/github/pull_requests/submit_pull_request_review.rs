//! `github.submit_pull_request_review`: a pending review of a pull request, submitted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::SubmitPullRequestReview, types::GithubReview};

impl UseCases {
    /// Runs `github.submit_pull_request_review` through the connection, refused first when it lacks
    /// [`SubmitPullRequestReview::SCOPES`].
    pub async fn github_submit_pull_request_review(
        &self,
        args: GithubActionArgs<'_, SubmitPullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_submit_pull_request_review", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.submit_pull_request_review",
                required_scopes: SubmitPullRequestReview::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .submit_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
