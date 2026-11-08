//! `github.update_pull_request`: a pull request's title, text, state, base branch or whether maintainers may push to it, changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::UpdatePullRequest, types::GithubPullRequest};

impl UseCases {
    /// Runs `github.update_pull_request` through the connection, refused first when it lacks
    /// [`UpdatePullRequest::SCOPES`].
    pub async fn github_update_pull_request(
        &self,
        args: GithubActionArgs<'_, UpdatePullRequest>,
    ) -> UseCaseResult<GithubPullRequest> {
        self.run("github_update_pull_request", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_pull_request",
                required_scopes: UpdatePullRequest::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_pull_request(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
