//! `github.merge_pull_request`: a pull request merged into its base branch.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, pull_requests::MergePullRequest, types::GithubMergeResult};

impl UseCases {
    /// Runs `github.merge_pull_request` through the connection, refused first when it lacks
    /// [`MergePullRequest::SCOPES`].
    pub async fn github_merge_pull_request(
        &self,
        args: ActionArgs<'_, MergePullRequest>,
    ) -> UseCaseResult<GithubMergeResult> {
        self.run("github_merge_pull_request", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.merge_pull_request",
                required_scopes: MergePullRequest::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .merge_pull_request(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
