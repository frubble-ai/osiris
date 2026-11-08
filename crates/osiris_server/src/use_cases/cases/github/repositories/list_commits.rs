//! `github.list_commits`: a repository's commits, filtered by branch, path, author and date, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListCommits, types::GithubCommit};

impl UseCases {
    /// Runs `github.list_commits` through the connection, refused first when it lacks
    /// [`ListCommits::SCOPES`].
    pub async fn github_list_commits(
        &self,
        args: GithubActionArgs<'_, ListCommits>,
    ) -> UseCaseResult<Vec<GithubCommit>> {
        self.run("github_list_commits", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_commits",
                required_scopes: ListCommits::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
