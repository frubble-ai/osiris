//! `github.create_issue_comment`: a new comment on an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::CreateIssueComment, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.create_issue_comment` through the connection, refused first when it lacks
    /// [`CreateIssueComment::SCOPES`].
    pub async fn github_create_issue_comment(
        &self,
        args: GithubActionArgs<'_, CreateIssueComment>,
    ) -> UseCaseResult<GithubIssueComment> {
        self.run("github_create_issue_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue_comment",
                required_scopes: CreateIssueComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
