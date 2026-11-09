//! `github.rename_branch`: a branch given a new name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::RenameBranch, types::GithubBranch};

impl UseCases {
    /// Runs `github.rename_branch` through the connection, refused first when it lacks
    /// [`RenameBranch::SCOPES`].
    pub async fn github_rename_branch(
        &self,
        args: ActionArgs<'_, RenameBranch>,
    ) -> UseCaseResult<GithubBranch> {
        self.run("github_rename_branch", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.rename_branch",
                required_scopes: RenameBranch::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .rename_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
