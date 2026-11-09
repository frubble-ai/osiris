//! `github.compare_commits`: two commits compared, with the commits and files between them.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::CompareCommits, types::GithubCommitComparison,
};

impl UseCases {
    /// Runs `github.compare_commits` through the connection, refused first when it lacks
    /// [`CompareCommits::SCOPES`].
    pub async fn github_compare_commits(
        &self,
        args: ActionArgs<'_, CompareCommits>,
    ) -> UseCaseResult<GithubCommitComparison> {
        self.run("github_compare_commits", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.compare_commits",
                required_scopes: CompareCommits::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .compare_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
