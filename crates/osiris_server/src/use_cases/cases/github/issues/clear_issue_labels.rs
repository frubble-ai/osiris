//! `github.clear_issue_labels`: every label taken off an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ClearIssueLabels, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.clear_issue_labels` through the connection, refused first when it lacks
    /// [`ClearIssueLabels::SCOPES`].
    pub async fn github_clear_issue_labels(
        &self,
        args: ActionArgs<'_, ClearIssueLabels>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_clear_issue_labels", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.clear_issue_labels",
                required_scopes: ClearIssueLabels::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .clear_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
