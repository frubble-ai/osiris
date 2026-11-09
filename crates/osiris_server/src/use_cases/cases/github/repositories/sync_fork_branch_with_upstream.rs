//! `github.sync_fork_branch_with_upstream`: a fork's branch brought up to date with its upstream.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::SyncForkBranchWithUpstream, types::GithubForkSync,
};

impl UseCases {
    /// Runs `github.sync_fork_branch_with_upstream` through the connection, refused first when it lacks
    /// [`SyncForkBranchWithUpstream::SCOPES`].
    pub async fn github_sync_fork_branch_with_upstream(
        &self,
        args: ActionArgs<'_, SyncForkBranchWithUpstream>,
    ) -> UseCaseResult<GithubForkSync> {
        self.run("github_sync_fork_branch_with_upstream", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.sync_fork_branch_with_upstream",
                required_scopes: SyncForkBranchWithUpstream::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .sync_fork_branch_with_upstream(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
