//! `github.list_pull_request_review_comments`: the comments on a pull request's diff, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ListPullRequestReviewComments, types::GithubReviewComment,
};

impl UseCases {
    /// Runs `github.list_pull_request_review_comments` through the connection, refused first when it lacks
    /// [`ListPullRequestReviewComments::SCOPES`].
    pub async fn github_list_pull_request_review_comments(
        &self,
        args: ActionArgs<'_, ListPullRequestReviewComments>,
    ) -> UseCaseResult<Vec<GithubReviewComment>> {
        self.run("github_list_pull_request_review_comments", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_review_comments",
                required_scopes: ListPullRequestReviewComments::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_pull_request_review_comments(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
