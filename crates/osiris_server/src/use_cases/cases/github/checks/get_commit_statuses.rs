//! `github.get_commit_statuses`: the statuses set on a commit, newest first, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, checks::GetCommitStatuses, types::GithubCommitStatus};

impl UseCases {
    /// Runs `github.get_commit_statuses` through the connection, refused first when it lacks
    /// [`GetCommitStatuses::SCOPES`].
    pub async fn github_get_commit_statuses(
        &self,
        args: ActionArgs<'_, GetCommitStatuses>,
    ) -> UseCaseResult<Vec<GithubCommitStatus>> {
        self.run("github_get_commit_statuses", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_commit_statuses",
                required_scopes: GetCommitStatuses::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_commit_statuses(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
