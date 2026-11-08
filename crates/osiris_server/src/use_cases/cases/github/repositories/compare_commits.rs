//! `github.compare_commits`: two commits compared, with the commits and files between them.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::CompareCommits, types::GithubCommitComparison};

impl UseCases {
    /// Runs `github.compare_commits` through the connection, refused first when it lacks
    /// [`CompareCommits::SCOPES`].
    pub async fn github_compare_commits(
        &self,
        args: GithubActionArgs<'_, CompareCommits>,
    ) -> UseCaseResult<GithubCommitComparison> {
        self.run("github_compare_commits", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.compare_commits",
                required_scopes: CompareCommits::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .compare_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
