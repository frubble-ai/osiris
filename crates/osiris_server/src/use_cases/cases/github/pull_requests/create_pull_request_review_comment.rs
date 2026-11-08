//! `github.create_pull_request_review_comment`: a comment on a line, or a range of lines, of a pull request's diff.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::CreatePullRequestReviewComment, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.create_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`CreatePullRequestReviewComment::SCOPES`].
    pub async fn github_create_pull_request_review_comment(
        &self,
        args: GithubActionArgs<'_, CreatePullRequestReviewComment>,
    ) -> UseCaseResult<GithubReviewComment> {
        self.run("github_create_pull_request_review_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_pull_request_review_comment",
                required_scopes: CreatePullRequestReviewComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
