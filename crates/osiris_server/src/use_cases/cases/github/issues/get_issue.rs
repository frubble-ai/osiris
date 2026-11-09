//! `github.get_issue`: an issue, by number.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::GetIssue, types::GithubIssue};

impl UseCases {
    /// Runs `github.get_issue` through the connection, refused first when it lacks
    /// [`GetIssue::SCOPES`].
    pub async fn github_get_issue(
        &self,
        args: ActionArgs<'_, GetIssue>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_get_issue", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_issue",
                required_scopes: GetIssue::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider.get_issue(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
