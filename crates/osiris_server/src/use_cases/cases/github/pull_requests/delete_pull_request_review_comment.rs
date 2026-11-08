//! `github.delete_pull_request_review_comment`: a comment on a pull request's diff, deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::DeletePullRequestReviewComment, types::GithubAcknowledgement,
};

impl UseCases {
    /// Runs `github.delete_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`DeletePullRequestReviewComment::SCOPES`].
    pub async fn github_delete_pull_request_review_comment(
        &self,
        args: GithubActionArgs<'_, DeletePullRequestReviewComment>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_pull_request_review_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_pull_request_review_comment",
                required_scopes: DeletePullRequestReviewComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
