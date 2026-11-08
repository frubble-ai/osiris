//! `github.create_issue`: a new issue in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::CreateIssue, types::GithubIssue};

impl UseCases {
    /// Runs `github.create_issue` through the connection, refused first when it lacks
    /// [`CreateIssue::SCOPES`].
    pub async fn github_create_issue(
        &self,
        args: GithubActionArgs<'_, CreateIssue>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_create_issue", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue",
                required_scopes: CreateIssue::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
