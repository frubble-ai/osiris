//! `github.delete_pending_pull_request_review`: a pending review of a pull request, deleted; it answers the review deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::DeletePendingPullRequestReview, types::GithubReview,
};

impl UseCases {
    /// Runs `github.delete_pending_pull_request_review` through the connection, refused first when it lacks
    /// [`DeletePendingPullRequestReview::SCOPES`].
    pub async fn github_delete_pending_pull_request_review(
        &self,
        args: GithubActionArgs<'_, DeletePendingPullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_delete_pending_pull_request_review", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_pending_pull_request_review",
                required_scopes: DeletePendingPullRequestReview::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_pending_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
