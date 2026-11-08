//! `github.create_issue`: a new issue in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::CreateIssue, types::GithubIssue};

impl UseCases {
    /// Runs `github.create_issue` through the connection, refused first when it lacks
    /// [`CreateIssue::SCOPES`].
    pub async fn github_create_issue(
        &self,
        args: ActionArgs<'_, CreateIssue>,
    ) -> UseCaseResult<GithubIssue> {
        self.run("github_create_issue", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue",
                required_scopes: CreateIssue::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_issue(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
