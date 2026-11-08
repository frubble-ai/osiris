//! `github.update_issue`: an issue's title, text, state, assignees, labels or milestone changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::UpdateIssue, types::GithubIssue};

impl UseCases {
    /// Runs `github.update_issue` through the connection, refused first when it lacks
    /// [`UpdateIssue::SCOPES`].
    pub async fn github_update_issue(
        &self,
        args: GithubActionArgs<'_, UpdateIssue>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_update_issue", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_issue",
                required_scopes: UpdateIssue::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
