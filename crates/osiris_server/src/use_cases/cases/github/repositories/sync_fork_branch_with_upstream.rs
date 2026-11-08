//! `github.sync_fork_branch_with_upstream`: a fork's branch brought up to date with its upstream.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::SyncForkBranchWithUpstream, types::GithubForkSync};

impl UseCases {
    /// Runs `github.sync_fork_branch_with_upstream` through the connection, refused first when it lacks
    /// [`SyncForkBranchWithUpstream::SCOPES`].
    pub async fn github_sync_fork_branch_with_upstream(
        &self,
        args: GithubActionArgs<'_, SyncForkBranchWithUpstream>,
    ) -> UseCaseResult<GithubForkSync> {
        self.run("github_sync_fork_branch_with_upstream", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.sync_fork_branch_with_upstream",
                required_scopes: SyncForkBranchWithUpstream::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .sync_fork_branch_with_upstream(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
