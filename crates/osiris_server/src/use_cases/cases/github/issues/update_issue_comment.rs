//! `github.update_issue_comment`: a comment on an issue, its text replaced.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::UpdateIssueComment, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.update_issue_comment` through the connection, refused first when it lacks
    /// [`UpdateIssueComment::SCOPES`].
    pub async fn github_update_issue_comment(
        &self,
        args: GithubActionArgs<'_, UpdateIssueComment>,
    ) -> UseCaseResult<GithubIssueComment> {
        self.run("github_update_issue_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_issue_comment",
                required_scopes: UpdateIssueComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
