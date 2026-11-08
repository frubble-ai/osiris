//! `github.create_issue_comment_reaction`: a reaction to a comment on an issue.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::CreateIssueCommentReaction, types::GithubReaction};

impl UseCases {
    /// Runs `github.create_issue_comment_reaction` through the connection, refused first when it lacks
    /// [`CreateIssueCommentReaction::SCOPES`].
    pub async fn github_create_issue_comment_reaction(
        &self,
        args: GithubActionArgs<'_, CreateIssueCommentReaction>,
    ) -> UseCaseResult<GithubReaction> {
        self.run("github_create_issue_comment_reaction", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_issue_comment_reaction",
                required_scopes: CreateIssueCommentReaction::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_issue_comment_reaction(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
