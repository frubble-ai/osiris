//! `github.update_pull_request_branch`: a pull request's branch brought up to date with the latest changes of its base branch.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::UpdatePullRequestBranch, types::GithubPullRequestBranchUpdate,
};

impl UseCases {
    /// Runs `github.update_pull_request_branch` through the connection, refused first when it lacks
    /// [`UpdatePullRequestBranch::SCOPES`].
    pub async fn github_update_pull_request_branch(
        &self,
        args: ActionArgs<'_, UpdatePullRequestBranch>,
    ) -> UseCaseResult<GithubPullRequestBranchUpdate> {
        self.run("github_update_pull_request_branch", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_pull_request_branch",
                required_scopes: UpdatePullRequestBranch::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_pull_request_branch(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
