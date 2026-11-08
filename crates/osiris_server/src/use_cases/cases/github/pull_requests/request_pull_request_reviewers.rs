//! `github.request_pull_request_reviewers`: a pull request's review requested from users and teams.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::RequestPullRequestReviewers, types::GithubReviewRequest,
};

impl UseCases {
    /// Runs `github.request_pull_request_reviewers` through the connection, refused first when it lacks
    /// [`RequestPullRequestReviewers::SCOPES`].
    pub async fn github_request_pull_request_reviewers(
        &self,
        args: ActionArgs<'_, RequestPullRequestReviewers>,
    ) -> UseCaseResult<GithubReviewRequest> {
        self.run("github_request_pull_request_reviewers", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.request_pull_request_reviewers",
                required_scopes: RequestPullRequestReviewers::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .request_pull_request_reviewers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
