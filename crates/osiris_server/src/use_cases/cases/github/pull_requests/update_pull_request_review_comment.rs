//! `github.update_pull_request_review_comment`: a comment on a pull request's diff, its text replaced.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::UpdatePullRequestReviewComment, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.update_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`UpdatePullRequestReviewComment::SCOPES`].
    pub async fn github_update_pull_request_review_comment(
        &self,
        args: ActionArgs<'_, UpdatePullRequestReviewComment>,
    ) -> UseCaseResult<GithubReviewComment> {
        self.run("github_update_pull_request_review_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_pull_request_review_comment",
                required_scopes: UpdatePullRequestReviewComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
