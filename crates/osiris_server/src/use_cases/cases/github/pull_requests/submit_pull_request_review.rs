//! `github.submit_pull_request_review`: a pending review of a pull request, submitted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::SubmitPullRequestReview, types::GithubReview,
};

impl UseCases {
    /// Runs `github.submit_pull_request_review` through the connection, refused first when it lacks
    /// [`SubmitPullRequestReview::SCOPES`].
    pub async fn github_submit_pull_request_review(
        &self,
        args: ActionArgs<'_, SubmitPullRequestReview>,
    ) -> UseCaseResult<GithubReview> {
        self.run("github_submit_pull_request_review", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.submit_pull_request_review",
                required_scopes: SubmitPullRequestReview::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .submit_pull_request_review(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
