//! `github.remove_issue_label`: one label taken off an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::RemoveIssueLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.remove_issue_label` through the connection, refused first when it lacks
    /// [`RemoveIssueLabel::SCOPES`].
    pub async fn github_remove_issue_label(
        &self,
        args: ActionArgs<'_, RemoveIssueLabel>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_remove_issue_label", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_issue_label",
                required_scopes: RemoveIssueLabel::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .remove_issue_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
