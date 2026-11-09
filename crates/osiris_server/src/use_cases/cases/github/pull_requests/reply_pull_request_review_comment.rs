//! `github.reply_pull_request_review_comment`: a reply to a comment on a pull request's diff.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ReplyPullRequestReviewComment, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.reply_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`ReplyPullRequestReviewComment::SCOPES`].
    pub async fn github_reply_pull_request_review_comment(
        &self,
        args: ActionArgs<'_, ReplyPullRequestReviewComment>,
    ) -> UseCaseResult<GithubReviewComment> {
        self.run("github_reply_pull_request_review_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.reply_pull_request_review_comment",
                required_scopes: ReplyPullRequestReviewComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .reply_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
