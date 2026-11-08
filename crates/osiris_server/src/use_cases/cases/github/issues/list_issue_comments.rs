//! `github.list_issue_comments`: the comments on an issue, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ListIssueComments, types::GithubIssueComment};

impl UseCases {
    /// Runs `github.list_issue_comments` through the connection, refused first when it lacks
    /// [`ListIssueComments::SCOPES`].
    pub async fn github_list_issue_comments(
        &self,
        args: ActionArgs<'_, ListIssueComments>,
    ) -> UseCaseResult<Vec<GithubIssueComment>> {
        self.run("github_list_issue_comments", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_comments",
                required_scopes: ListIssueComments::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_issue_comments(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
