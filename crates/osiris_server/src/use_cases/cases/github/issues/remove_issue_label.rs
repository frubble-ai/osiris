//! `github.remove_issue_label`: one label taken off an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::RemoveIssueLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.remove_issue_label` through the connection, refused first when it lacks
    /// [`RemoveIssueLabel::SCOPES`].
    pub async fn github_remove_issue_label(
        &self,
        args: GithubActionArgs<'_, RemoveIssueLabel>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_remove_issue_label", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_issue_label",
                required_scopes: RemoveIssueLabel::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .remove_issue_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
