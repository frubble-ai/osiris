//! `github.list_pull_requests_associated_with_commit`: the pull requests a commit belongs to, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ListPullRequestsAssociatedWithCommit, types::GithubPullRequest,
};

impl UseCases {
    /// Runs `github.list_pull_requests_associated_with_commit` through the connection, refused first when it lacks
    /// [`ListPullRequestsAssociatedWithCommit::SCOPES`].
    pub async fn github_list_pull_requests_associated_with_commit(
        &self,
        args: ActionArgs<'_, ListPullRequestsAssociatedWithCommit>,
    ) -> UseCaseResult<Vec<GithubPullRequest>> {
        self.run("github_list_pull_requests_associated_with_commit", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_requests_associated_with_commit",
                required_scopes: ListPullRequestsAssociatedWithCommit::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_pull_requests_associated_with_commit(
                            http,
                            token.expose_secret(),
                            input,
                        )
                        .await
                },
            })
            .await
        })
        .await
    }
}
