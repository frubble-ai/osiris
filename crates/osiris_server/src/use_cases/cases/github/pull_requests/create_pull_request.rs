//! `github.create_pull_request`: a new pull request, from one branch into another.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::CreatePullRequest, types::GithubPullRequest};

impl UseCases {
    /// Runs `github.create_pull_request` through the connection, refused first when it lacks
    /// [`CreatePullRequest::SCOPES`].
    pub async fn github_create_pull_request(
        &self,
        args: GithubActionArgs<'_, CreatePullRequest>,
    ) -> UseCaseResult<GithubPullRequest> {
        self.run("github_create_pull_request", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_pull_request",
                required_scopes: CreatePullRequest::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_pull_request(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
