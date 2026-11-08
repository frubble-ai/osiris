//! `github.delete_issue_comment`: a comment on an issue deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::DeleteIssueComment, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_issue_comment` through the connection, refused first when it lacks
    /// [`DeleteIssueComment::SCOPES`].
    pub async fn github_delete_issue_comment(
        &self,
        args: GithubActionArgs<'_, DeleteIssueComment>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_issue_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_issue_comment",
                required_scopes: DeleteIssueComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_issue_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
