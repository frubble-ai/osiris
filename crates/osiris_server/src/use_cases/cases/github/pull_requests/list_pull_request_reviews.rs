//! `github.list_pull_request_reviews`: the reviews of a pull request, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ListPullRequestReviews, types::GithubReview,
};

impl UseCases {
    /// Runs `github.list_pull_request_reviews` through the connection, refused first when it lacks
    /// [`ListPullRequestReviews::SCOPES`].
    pub async fn github_list_pull_request_reviews(
        &self,
        args: ActionArgs<'_, ListPullRequestReviews>,
    ) -> UseCaseResult<Vec<GithubReview>> {
        self.run("github_list_pull_request_reviews", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_reviews",
                required_scopes: ListPullRequestReviews::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_pull_request_reviews(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
