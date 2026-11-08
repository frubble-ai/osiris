//! `github.list_pull_requests_associated_with_commit`: the pull requests a commit belongs to, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    pull_requests::ListPullRequestsAssociatedWithCommit, types::GithubPullRequest,
};

impl UseCases {
    /// Runs `github.list_pull_requests_associated_with_commit` through the connection, refused first when it lacks
    /// [`ListPullRequestsAssociatedWithCommit::SCOPES`].
    pub async fn github_list_pull_requests_associated_with_commit(
        &self,
        args: GithubActionArgs<'_, ListPullRequestsAssociatedWithCommit>,
    ) -> UseCaseResult<Vec<GithubPullRequest>> {
        self.run("github_list_pull_requests_associated_with_commit", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_requests_associated_with_commit",
                required_scopes: ListPullRequestsAssociatedWithCommit::SCOPES,
                call: move |token: Secret| async move {
                    github
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
