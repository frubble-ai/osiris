//! `github.delete_pull_request_review_comment`: a comment on a pull request's diff, deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::DeletePullRequestReviewComment, types::GithubAcknowledgement,
};

impl UseCases {
    /// Runs `github.delete_pull_request_review_comment` through the connection, refused first when it lacks
    /// [`DeletePullRequestReviewComment::SCOPES`].
    pub async fn github_delete_pull_request_review_comment(
        &self,
        args: ActionArgs<'_, DeletePullRequestReviewComment>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_pull_request_review_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_pull_request_review_comment",
                required_scopes: DeletePullRequestReviewComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_pull_request_review_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
