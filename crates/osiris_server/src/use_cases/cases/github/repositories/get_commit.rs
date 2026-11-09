//! `github.get_commit`: a commit, with its stats and the files it changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::GetCommit, types::GithubCommit};

impl UseCases {
    /// Runs `github.get_commit` through the connection, refused first when it lacks
    /// [`GetCommit::SCOPES`].
    pub async fn github_get_commit(
        &self,
        args: ActionArgs<'_, GetCommit>,
    ) -> UseCaseResult<GithubCommit> {
        self.run("github_get_commit", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_commit",
                required_scopes: GetCommit::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_commit(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
