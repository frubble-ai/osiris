//! `github.create_issue_comment`: a new comment on an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::CreateIssueComment, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.create_issue_comment` through the connection, refused first when it lacks
    /// [`CreateIssueComment::SCOPES`].
    pub async fn github_create_issue_comment(
        &self,
        args: ActionArgs<'_, CreateIssueComment>,
    ) -> UseCaseResult<GithubIssueComment> {
        self.run("github_create_issue_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue_comment",
                required_scopes: CreateIssueComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
