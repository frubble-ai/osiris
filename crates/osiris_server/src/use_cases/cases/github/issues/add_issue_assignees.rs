//! `github.add_issue_assignees`: people assigned to an issue, beside those assigned now.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::AddIssueAssignees, types::GithubIssue};

impl UseCases {
    /// Runs `github.add_issue_assignees` through the connection, refused first when it lacks
    /// [`AddIssueAssignees::SCOPES`].
    pub async fn github_add_issue_assignees(
        &self,
        args: ActionArgs<'_, AddIssueAssignees>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_add_issue_assignees", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.add_issue_assignees",
                required_scopes: AddIssueAssignees::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .add_issue_assignees(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
