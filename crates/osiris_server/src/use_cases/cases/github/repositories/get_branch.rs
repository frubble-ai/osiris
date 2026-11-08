//! `github.get_branch`: a branch, by name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::GetBranch, types::GithubBranch};

impl UseCases {
    /// Runs `github.get_branch` through the connection, refused first when it lacks
    /// [`GetBranch::SCOPES`].
    pub async fn github_get_branch(
        &self,
        args: ActionArgs<'_, GetBranch>,
    ) -> UseCaseResult<GithubBranch> {
        self.run("github_get_branch", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_branch",
                required_scopes: GetBranch::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
