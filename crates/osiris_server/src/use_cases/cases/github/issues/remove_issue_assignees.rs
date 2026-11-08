//! `github.remove_issue_assignees`: people no longer assigned to an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::RemoveIssueAssignees, types::GithubIssue};

impl UseCases {
    /// Runs `github.remove_issue_assignees` through the connection, refused first when it lacks
    /// [`RemoveIssueAssignees::SCOPES`].
    pub async fn github_remove_issue_assignees(
        &self,
        args: GithubActionArgs<'_, RemoveIssueAssignees>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_remove_issue_assignees", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_issue_assignees",
                required_scopes: RemoveIssueAssignees::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .remove_issue_assignees(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
