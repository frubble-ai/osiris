//! `github.reply_pull_request_review_comment`: a reply to a comment on a pull request's diff.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::ReplyPullRequestReviewComment, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.reply_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`ReplyPullRequestReviewComment::SCOPES`].
    pub async fn github_reply_pull_request_review_comment(
        &self,
        args: GithubActionArgs<'_, ReplyPullRequestReviewComment>,
    ) -> UseCaseResult<GithubReviewComment> {
        self.run("github_reply_pull_request_review_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.reply_pull_request_review_comment",
                required_scopes: ReplyPullRequestReviewComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .reply_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
