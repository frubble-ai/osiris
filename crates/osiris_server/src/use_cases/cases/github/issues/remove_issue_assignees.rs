//! `github.remove_issue_assignees`: people no longer assigned to an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::RemoveIssueAssignees, types::GithubIssue};

impl UseCases {
    /// Runs `github.remove_issue_assignees` through the connection, refused first when it lacks
    /// [`RemoveIssueAssignees::SCOPES`].
    pub async fn github_remove_issue_assignees(
        &self,
        args: ActionArgs<'_, RemoveIssueAssignees>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_remove_issue_assignees", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_issue_assignees",
                required_scopes: RemoveIssueAssignees::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .remove_issue_assignees(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
