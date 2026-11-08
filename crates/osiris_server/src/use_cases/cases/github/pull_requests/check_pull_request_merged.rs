//! `github.check_pull_request_merged`: whether a pull request has been merged.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::CheckPullRequestMerged, types::GithubMergedCheck,
};

impl UseCases {
    /// Runs `github.check_pull_request_merged` through the connection, refused first when it lacks
    /// [`CheckPullRequestMerged::SCOPES`].
    pub async fn github_check_pull_request_merged(
        &self,
        args: ActionArgs<'_, CheckPullRequestMerged>,
    ) -> UseCaseResult<GithubMergedCheck> {
        self.run("github_check_pull_request_merged", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.check_pull_request_merged",
                required_scopes: CheckPullRequestMerged::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .check_pull_request_merged(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
