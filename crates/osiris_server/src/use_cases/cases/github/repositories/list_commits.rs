//! `github.list_commits`: a repository's commits, filtered by branch, path, author and date, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::ListCommits, types::GithubCommit};

impl UseCases {
    /// Runs `github.list_commits` through the connection, refused first when it lacks
    /// [`ListCommits::SCOPES`].
    pub async fn github_list_commits(
        &self,
        args: ActionArgs<'_, ListCommits>,
    ) -> UseCaseResult<Vec<GithubCommit>> {
        self.run("github_list_commits", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_commits",
                required_scopes: ListCommits::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_commits(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
