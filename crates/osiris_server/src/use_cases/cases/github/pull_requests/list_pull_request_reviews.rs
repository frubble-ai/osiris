//! `github.list_pull_request_reviews`: the reviews of a pull request, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::ListPullRequestReviews, types::GithubReview};

impl UseCases {
    /// Runs `github.list_pull_request_reviews` through the connection, refused first when it lacks
    /// [`ListPullRequestReviews::SCOPES`].
    pub async fn github_list_pull_request_reviews(
        &self,
        args: GithubActionArgs<'_, ListPullRequestReviews>,
    ) -> UseCaseResult<Vec<GithubReview>> {
        self.run("github_list_pull_request_reviews", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_reviews",
                required_scopes: ListPullRequestReviews::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_pull_request_reviews(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
