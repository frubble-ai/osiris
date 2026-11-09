//! `github.update_issue`: an issue's title, text, state, assignees, labels or milestone changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::UpdateIssue, types::GithubIssue};

impl UseCases {
    /// Runs `github.update_issue` through the connection, refused first when it lacks
    /// [`UpdateIssue::SCOPES`].
    pub async fn github_update_issue(
        &self,
        args: ActionArgs<'_, UpdateIssue>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_update_issue", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_issue",
                required_scopes: UpdateIssue::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
