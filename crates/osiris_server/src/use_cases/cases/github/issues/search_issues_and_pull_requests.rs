//! `github.search_issues_and_pull_requests`: issues and pull requests found with GitHub's search syntax, structured filters, or both.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, issues::SearchIssuesAndPullRequests, types::GithubSearchIssueResults,
};

impl UseCases {
    /// Runs `github.search_issues_and_pull_requests` through the connection, refused first when it lacks
    /// [`SearchIssuesAndPullRequests::SCOPES`].
    pub async fn github_search_issues_and_pull_requests(
        &self,
        args: ActionArgs<'_, SearchIssuesAndPullRequests>,
    ) -> UseCaseResult<GithubSearchIssueResults> {
        self.run("github_search_issues_and_pull_requests", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_issues_and_pull_requests",
                required_scopes: SearchIssuesAndPullRequests::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_issues_and_pull_requests(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
