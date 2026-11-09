//! `github.create_pull_request_review`: a review of a pull request, with comments on lines of its diff if given.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::CreatePullRequestReview, types::GithubReview,
};

impl UseCases {
    /// Runs `github.create_pull_request_review` through the connection, refused first when it lacks
    /// [`CreatePullRequestReview::SCOPES`].
    pub async fn github_create_pull_request_review(
        &self,
        args: ActionArgs<'_, CreatePullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_create_pull_request_review", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_pull_request_review",
                required_scopes: CreatePullRequestReview::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
