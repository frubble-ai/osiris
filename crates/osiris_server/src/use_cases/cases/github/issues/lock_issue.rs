//! `github.lock_issue`: an issue's conversation locked, so only collaborators can comment.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::LockIssue, types::GithubIssueLock};

impl UseCases {
    /// Runs `github.lock_issue` through the connection, refused first when it lacks
    /// [`LockIssue::SCOPES`].
    pub async fn github_lock_issue(
        &self,
        args: GithubActionArgs<'_, LockIssue>,
    ) -> UseCaseResult<GithubIssueLock> {
        self.run("github_lock_issue", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.lock_issue",
                required_scopes: LockIssue::SCOPES,
                call: move |token: Secret| async move {
                    github.lock_issue(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
