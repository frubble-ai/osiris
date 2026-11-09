//! `github.create_pull_request`: a new pull request, from one branch into another.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::CreatePullRequest, types::GithubPullRequest,
};

impl UseCases {
    /// Runs `github.create_pull_request` through the connection, refused first when it lacks
    /// [`CreatePullRequest::SCOPES`].
    pub async fn github_create_pull_request(
        &self,
        args: ActionArgs<'_, CreatePullRequest>,
    ) -> UseCaseResult<GithubPullRequest> {
        self.run("github_create_pull_request", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_pull_request",
                required_scopes: CreatePullRequest::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_pull_request(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
