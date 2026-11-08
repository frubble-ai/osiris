//! `github.unlock_issue`: an issue's conversation unlocked.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::UnlockIssue, types::GithubIssueLock};

impl UseCases {
    /// Runs `github.unlock_issue` through the connection, refused first when it lacks
    /// [`UnlockIssue::SCOPES`].
    pub async fn github_unlock_issue(
        &self,
        args: GithubActionArgs<'_, UnlockIssue>,
    ) -> UseCaseResult<GithubIssueLock> {
        self.run("github_unlock_issue", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.unlock_issue",
                required_scopes: UnlockIssue::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .unlock_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
