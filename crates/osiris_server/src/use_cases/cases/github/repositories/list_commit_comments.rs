//! `github.list_commit_comments`: the comments on a commit, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListCommitComments, types::GithubCommitComment};

impl UseCases {
    /// Runs `github.list_commit_comments` through the connection, refused first when it lacks
    /// [`ListCommitComments::SCOPES`].
    pub async fn github_list_commit_comments(
        &self,
        args: GithubActionArgs<'_, ListCommitComments>,
    ) -> UseCaseResult<Vec<GithubCommitComment>> {
        self.run("github_list_commit_comments", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_commit_comments",
                required_scopes: ListCommitComments::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_commit_comments(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
