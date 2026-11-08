//! `github.search_issues_and_pull_requests`: issues and pull requests found with GitHub's search syntax, structured filters, or both.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    issues::SearchIssuesAndPullRequests, types::GithubSearchIssueResults,
};

impl UseCases {
    /// Runs `github.search_issues_and_pull_requests` through the connection, refused first when it lacks
    /// [`SearchIssuesAndPullRequests::SCOPES`].
    pub async fn github_search_issues_and_pull_requests(
        &self,
        args: GithubActionArgs<'_, SearchIssuesAndPullRequests>,
    ) -> UseCaseResult<GithubSearchIssueResults> {
        self.run("github_search_issues_and_pull_requests", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_issues_and_pull_requests",
                required_scopes: SearchIssuesAndPullRequests::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .search_issues_and_pull_requests(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
