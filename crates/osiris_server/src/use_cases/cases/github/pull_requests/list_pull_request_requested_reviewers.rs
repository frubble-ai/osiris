//! `github.list_pull_request_requested_reviewers`: the users and teams a pull request's review is requested from.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::ListPullRequestRequestedReviewers, types::GithubRequestedReviewers,
};

impl UseCases {
    /// Runs `github.list_pull_request_requested_reviewers` through the connection, refused first when it lacks
    /// [`ListPullRequestRequestedReviewers::SCOPES`].
    pub async fn github_list_pull_request_requested_reviewers(
        &self,
        args: GithubActionArgs<'_, ListPullRequestRequestedReviewers>,
    ) -> UseCaseResult<GithubRequestedReviewers> {
        self.run("github_list_pull_request_requested_reviewers", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_requested_reviewers",
                required_scopes: ListPullRequestRequestedReviewers::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_pull_request_requested_reviewers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
