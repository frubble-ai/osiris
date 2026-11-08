//! `github.add_issue_assignees`: people assigned to an issue, beside those assigned now.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::AddIssueAssignees, types::GithubIssue};

impl UseCases {
    /// Runs `github.add_issue_assignees` through the connection, refused first when it lacks
    /// [`AddIssueAssignees::SCOPES`].
    pub async fn github_add_issue_assignees(
        &self,
        args: GithubActionArgs<'_, AddIssueAssignees>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_add_issue_assignees", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.add_issue_assignees",
                required_scopes: AddIssueAssignees::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .add_issue_assignees(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
