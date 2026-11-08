//! `github.create_commit_status`: sets a status on a commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{checks::CreateCommitStatus, types::GithubCommitStatus};

impl UseCases {
    /// Runs `github.create_commit_status` through the connection, refused first when it lacks
    /// [`CreateCommitStatus::SCOPES`].
    pub async fn github_create_commit_status(
        &self,
        args: GithubActionArgs<'_, CreateCommitStatus>,
    ) -> UseCaseResult<GithubCommitStatus> {
        self.run("github_create_commit_status", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_commit_status",
                required_scopes: CreateCommitStatus::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_commit_status(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
