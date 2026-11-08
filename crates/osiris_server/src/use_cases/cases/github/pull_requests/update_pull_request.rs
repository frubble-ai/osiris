//! `github.update_pull_request`: a pull request's title, text, state, base branch or whether maintainers may push to it, changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::UpdatePullRequest, types::GithubPullRequest,
};

impl UseCases {
    /// Runs `github.update_pull_request` through the connection, refused first when it lacks
    /// [`UpdatePullRequest::SCOPES`].
    pub async fn github_update_pull_request(
        &self,
        args: ActionArgs<'_, UpdatePullRequest>,
    ) -> UseCaseResult<GithubPullRequest> {
        self.run("github_update_pull_request", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_pull_request",
                required_scopes: UpdatePullRequest::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_pull_request(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
