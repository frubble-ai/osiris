//! `github.create_commit_status`: sets a status on a commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, checks::CreateCommitStatus, types::GithubCommitStatus};

impl UseCases {
    /// Runs `github.create_commit_status` through the connection, refused first when it lacks
    /// [`CreateCommitStatus::SCOPES`].
    pub async fn github_create_commit_status(
        &self,
        args: ActionArgs<'_, CreateCommitStatus>,
    ) -> UseCaseResult<GithubCommitStatus> {
        self.run("github_create_commit_status", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_commit_status",
                required_scopes: CreateCommitStatus::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_commit_status(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
