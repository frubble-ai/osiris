//! `github.list_pull_request_commits`: the commits on a pull request, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::ListPullRequestCommits, types::GithubCommit};

impl UseCases {
    /// Runs `github.list_pull_request_commits` through the connection, refused first when it lacks
    /// [`ListPullRequestCommits::SCOPES`].
    pub async fn github_list_pull_request_commits(
        &self,
        args: GithubActionArgs<'_, ListPullRequestCommits>,
    ) -> UseCaseResult<Vec<GithubCommit>> {
        self.run("github_list_pull_request_commits", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_commits",
                required_scopes: ListPullRequestCommits::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_pull_request_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
