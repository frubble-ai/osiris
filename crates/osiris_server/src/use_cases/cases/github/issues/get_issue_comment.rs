//! `github.get_issue_comment`: a comment on an issue, by id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::GetIssueComment, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.get_issue_comment` through the connection, refused first when it lacks
    /// [`GetIssueComment::SCOPES`].
    pub async fn github_get_issue_comment(
        &self,
        args: ActionArgs<'_, GetIssueComment>,
    ) -> UseCaseResult<GithubIssueComment> {
        self.run("github_get_issue_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_issue_comment",
                required_scopes: GetIssueComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
