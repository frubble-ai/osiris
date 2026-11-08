//! `github.update_issue_comment`: a comment on an issue, its text replaced.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::UpdateIssueComment, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.update_issue_comment` through the connection, refused first when it lacks
    /// [`UpdateIssueComment::SCOPES`].
    pub async fn github_update_issue_comment(
        &self,
        args: ActionArgs<'_, UpdateIssueComment>,
    ) -> UseCaseResult<GithubIssueComment> {
        self.run("github_update_issue_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_issue_comment",
                required_scopes: UpdateIssueComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
