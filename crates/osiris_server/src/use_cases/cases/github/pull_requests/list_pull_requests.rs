//! `github.list_pull_requests`: a repository's pull requests, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::ListPullRequests, types::GithubPullRequest};

impl UseCases {
    /// Runs `github.list_pull_requests` through the connection, refused first when it lacks
    /// [`ListPullRequests::SCOPES`].
    pub async fn github_list_pull_requests(
        &self,
        args: GithubActionArgs<'_, ListPullRequests>,
    ) -> UseCaseResult<Vec<GithubPullRequest>> {
        self.run("github_list_pull_requests", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_requests",
                required_scopes: ListPullRequests::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_pull_requests(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
