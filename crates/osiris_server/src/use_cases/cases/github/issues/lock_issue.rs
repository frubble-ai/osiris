//! `github.lock_issue`: an issue's conversation locked, so only collaborators can comment.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::LockIssue, types::GithubIssueLock};

impl UseCases {
    /// Runs `github.lock_issue` through the connection, refused first when it lacks
    /// [`LockIssue::SCOPES`].
    pub async fn github_lock_issue(
        &self,
        args: ActionArgs<'_, LockIssue>,
    ) -> UseCaseResult<GithubIssueLock> {
        self.run("github_lock_issue", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.lock_issue",
                required_scopes: LockIssue::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .lock_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
