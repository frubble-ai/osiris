//! `github.dismiss_pull_request_review`: a submitted review of a pull request, dismissed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::DismissPullRequestReview, types::GithubReview};

impl UseCases {
    /// Runs `github.dismiss_pull_request_review` through the connection, refused first when it lacks
    /// [`DismissPullRequestReview::SCOPES`].
    pub async fn github_dismiss_pull_request_review(
        &self,
        args: GithubActionArgs<'_, DismissPullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_dismiss_pull_request_review", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.dismiss_pull_request_review",
                required_scopes: DismissPullRequestReview::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .dismiss_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
