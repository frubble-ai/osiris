//! `github.list_pull_request_requested_reviewers`: the users and teams a pull request's review is requested from.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ListPullRequestRequestedReviewers, types::GithubRequestedReviewers,
};

impl UseCases {
    /// Runs `github.list_pull_request_requested_reviewers` through the connection, refused first when it lacks
    /// [`ListPullRequestRequestedReviewers::SCOPES`].
    pub async fn github_list_pull_request_requested_reviewers(
        &self,
        args: ActionArgs<'_, ListPullRequestRequestedReviewers>,
    ) -> UseCaseResult<GithubRequestedReviewers> {
        self.run("github_list_pull_request_requested_reviewers", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_requested_reviewers",
                required_scopes: ListPullRequestRequestedReviewers::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_pull_request_requested_reviewers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
