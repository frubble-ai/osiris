//! `github.list_issue_labels`: the labels on an issue, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListIssueLabels, types::GithubLabel};

impl UseCases {
    /// Runs `github.list_issue_labels` through the connection, refused first when it lacks
    /// [`ListIssueLabels::SCOPES`].
    pub async fn github_list_issue_labels(
        &self,
        args: GithubActionArgs<'_, ListIssueLabels>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_list_issue_labels", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_labels",
                required_scopes: ListIssueLabels::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
