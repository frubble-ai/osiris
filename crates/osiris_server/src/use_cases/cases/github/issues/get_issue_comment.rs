//! `github.get_issue_comment`: a comment on an issue, by id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::GetIssueComment, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.get_issue_comment` through the connection, refused first when it lacks
    /// [`GetIssueComment::SCOPES`].
    pub async fn github_get_issue_comment(
        &self,
        args: GithubActionArgs<'_, GetIssueComment>,
    ) -> UseCaseResult<GithubIssueComment> {
        self.run("github_get_issue_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_issue_comment",
                required_scopes: GetIssueComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
