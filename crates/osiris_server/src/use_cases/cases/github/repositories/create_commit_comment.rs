//! `github.create_commit_comment`: a comment on a commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::CreateCommitComment, types::GithubCommitComment,
};

impl UseCases {
    /// Runs `github.create_commit_comment` through the connection, refused first when it lacks
    /// [`CreateCommitComment::SCOPES`].
    pub async fn github_create_commit_comment(
        &self,
        args: ActionArgs<'_, CreateCommitComment>,
    ) -> UseCaseResult<GithubCommitComment> {
        self.run("github_create_commit_comment", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_commit_comment",
                required_scopes: CreateCommitComment::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_commit_comment(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
