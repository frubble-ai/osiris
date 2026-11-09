//! `github.list_commit_comments`: the comments on a commit, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::ListCommitComments, types::GithubCommitComment,
};

impl UseCases {
    /// Runs `github.list_commit_comments` through the connection, refused first when it lacks
    /// [`ListCommitComments::SCOPES`].
    pub async fn github_list_commit_comments(
        &self,
        args: ActionArgs<'_, ListCommitComments>,
    ) -> UseCaseResult<Vec<GithubCommitComment>> {
        self.run("github_list_commit_comments", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_commit_comments",
                required_scopes: ListCommitComments::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_commit_comments(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
