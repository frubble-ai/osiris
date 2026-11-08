//! `github.list_issue_comments`: the comments on an issue, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListIssueComments, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.list_issue_comments` through the connection, refused first when it lacks
    /// [`ListIssueComments::SCOPES`].
    pub async fn github_list_issue_comments(
        &self,
        args: GithubActionArgs<'_, ListIssueComments>,
    ) -> UseCaseResult<Vec<GithubIssueComment>> {
        self.run("github_list_issue_comments", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_comments",
                required_scopes: ListIssueComments::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_issue_comments(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
