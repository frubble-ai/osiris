//! `github.delete_issue_comment`: a comment on an issue deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::DeleteIssueComment, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_issue_comment` through the connection, refused first when it lacks
    /// [`DeleteIssueComment::SCOPES`].
    pub async fn github_delete_issue_comment(
        &self,
        args: ActionArgs<'_, DeleteIssueComment>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_issue_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_issue_comment",
                required_scopes: DeleteIssueComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
