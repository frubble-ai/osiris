//! `github.set_issue_labels`: an issue's labels replaced with those given.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::SetIssueLabels, types::GithubLabel};

impl UseCases {
    /// Runs `github.set_issue_labels` through the connection, refused first when it lacks
    /// [`SetIssueLabels::SCOPES`].
    pub async fn github_set_issue_labels(
        &self,
        args: GithubActionArgs<'_, SetIssueLabels>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_set_issue_labels", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.set_issue_labels",
                required_scopes: SetIssueLabels::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .set_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
