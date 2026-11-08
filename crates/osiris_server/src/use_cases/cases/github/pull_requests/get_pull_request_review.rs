//! `github.get_pull_request_review`: a review of a pull request, by id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::GetPullRequestReview, types::GithubReview};

impl UseCases {
    /// Runs `github.get_pull_request_review` through the connection, refused first when it lacks
    /// [`GetPullRequestReview::SCOPES`].
    pub async fn github_get_pull_request_review(
        &self,
        args: GithubActionArgs<'_, GetPullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_get_pull_request_review", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_pull_request_review",
                required_scopes: GetPullRequestReview::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
