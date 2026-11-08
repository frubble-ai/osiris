//! `github.add_issue_labels`: labels added to an issue, beside those it has.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::AddIssueLabels, types::GithubLabel};

impl UseCases {
    /// Runs `github.add_issue_labels` through the connection, refused first when it lacks
    /// [`AddIssueLabels::SCOPES`].
    pub async fn github_add_issue_labels(
        &self,
        args: GithubActionArgs<'_, AddIssueLabels>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_add_issue_labels", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.add_issue_labels",
                required_scopes: AddIssueLabels::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .add_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
