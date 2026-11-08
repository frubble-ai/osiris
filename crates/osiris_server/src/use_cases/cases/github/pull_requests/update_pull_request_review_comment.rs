//! `github.update_pull_request_review_comment`: a comment on a pull request's diff, its text replaced.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::UpdatePullRequestReviewComment, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.update_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`UpdatePullRequestReviewComment::SCOPES`].
    pub async fn github_update_pull_request_review_comment(
        &self,
        args: GithubActionArgs<'_, UpdatePullRequestReviewComment>,
    ) -> UseCaseResult<GithubReviewComment> {
        self.run("github_update_pull_request_review_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_pull_request_review_comment",
                required_scopes: UpdatePullRequestReviewComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
