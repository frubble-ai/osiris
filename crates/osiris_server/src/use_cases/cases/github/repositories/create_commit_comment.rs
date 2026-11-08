//! `github.create_commit_comment`: a comment on a commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::CreateCommitComment, types::GithubCommitComment};

impl UseCases {
    /// Runs `github.create_commit_comment` through the connection, refused first when it lacks
    /// [`CreateCommitComment::SCOPES`].
    pub async fn github_create_commit_comment(
        &self,
        args: GithubActionArgs<'_, CreateCommitComment>,
    ) -> UseCaseResult<GithubCommitComment> {
        self.run("github_create_commit_comment", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_commit_comment",
                required_scopes: CreateCommitComment::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_commit_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
