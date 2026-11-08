//! `github.list_pull_request_review_comments`: the comments on a pull request's diff, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::ListPullRequestReviewComments, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.list_pull_request_review_comments` through the connection, refused first when it lacks
    /// [`ListPullRequestReviewComments::SCOPES`].
    pub async fn github_list_pull_request_review_comments(
        &self,
        args: GithubActionArgs<'_, ListPullRequestReviewComments>,
    ) -> UseCaseResult<Vec<GithubReviewComment>> {
        self.run("github_list_pull_request_review_comments", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_review_comments",
                required_scopes: ListPullRequestReviewComments::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_pull_request_review_comments(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
