//! `github.get_commit_statuses`: the statuses set on a commit, newest first, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{checks::GetCommitStatuses, types::GithubCommitStatus};

impl UseCases {
    /// Runs `github.get_commit_statuses` through the connection, refused first when it lacks
    /// [`GetCommitStatuses::SCOPES`].
    pub async fn github_get_commit_statuses(
        &self,
        args: GithubActionArgs<'_, GetCommitStatuses>,
    ) -> UseCaseResult<Vec<GithubCommitStatus>> {
        self.run("github_get_commit_statuses", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_commit_statuses",
                required_scopes: GetCommitStatuses::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_commit_statuses(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
