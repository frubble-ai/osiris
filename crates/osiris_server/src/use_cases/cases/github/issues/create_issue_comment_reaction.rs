//! `github.create_issue_comment_reaction`: a reaction to a comment on an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::CreateIssueCommentReaction, types::GithubReaction};

impl UseCases {
    /// Runs `github.create_issue_comment_reaction` through the connection, refused first when it lacks
    /// [`CreateIssueCommentReaction::SCOPES`].
    pub async fn github_create_issue_comment_reaction(
        &self,
        args: ActionArgs<'_, CreateIssueCommentReaction>,
    ) -> UseCaseResult<GithubReaction> {
        self.run("github_create_issue_comment_reaction", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue_comment_reaction",
                required_scopes: CreateIssueCommentReaction::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_issue_comment_reaction(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
