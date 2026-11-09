//! `github.unlock_issue`: an issue's conversation unlocked.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::UnlockIssue, types::GithubIssueLock};

impl UseCases {
    /// Runs `github.unlock_issue` through the connection, refused first when it lacks
    /// [`UnlockIssue::SCOPES`].
    pub async fn github_unlock_issue(
        &self,
        args: ActionArgs<'_, UnlockIssue>,
    ) -> UseCaseResult<GithubIssueLock> {
        self.run("github_unlock_issue", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.unlock_issue",
                required_scopes: UnlockIssue::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .unlock_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
