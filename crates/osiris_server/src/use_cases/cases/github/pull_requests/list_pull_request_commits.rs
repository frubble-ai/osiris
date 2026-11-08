//! `github.list_pull_request_commits`: the commits on a pull request, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ListPullRequestCommits, types::GithubCommit,
};

impl UseCases {
    /// Runs `github.list_pull_request_commits` through the connection, refused first when it lacks
    /// [`ListPullRequestCommits::SCOPES`].
    pub async fn github_list_pull_request_commits(
        &self,
        args: ActionArgs<'_, ListPullRequestCommits>,
    ) -> UseCaseResult<Vec<GithubCommit>> {
        self.run("github_list_pull_request_commits", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_commits",
                required_scopes: ListPullRequestCommits::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_pull_request_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
