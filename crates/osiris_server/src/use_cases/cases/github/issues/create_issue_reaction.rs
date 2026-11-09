//! `github.create_issue_reaction`: a reaction to an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::CreateIssueReaction, types::GithubReaction};

impl UseCases {
    /// Runs `github.create_issue_reaction` through the connection, refused first when it lacks
    /// [`CreateIssueReaction::SCOPES`].
    pub async fn github_create_issue_reaction(
        &self,
        args: ActionArgs<'_, CreateIssueReaction>,
    ) -> UseCaseResult<GithubReaction> {
        self.run("github_create_issue_reaction", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue_reaction",
                required_scopes: CreateIssueReaction::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_issue_reaction(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
