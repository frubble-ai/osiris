//! `github.clear_issue_labels`: every label taken off an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ClearIssueLabels, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.clear_issue_labels` through the connection, refused first when it lacks
    /// [`ClearIssueLabels::SCOPES`].
    pub async fn github_clear_issue_labels(
        &self,
        args: GithubActionArgs<'_, ClearIssueLabels>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_clear_issue_labels", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.clear_issue_labels",
                required_scopes: ClearIssueLabels::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .clear_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
