//! `github.list_pull_request_files`: the files a pull request changes, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, pull_requests::ListPullRequestFiles, types::GithubPullRequestFile,
};

impl UseCases {
    /// Runs `github.list_pull_request_files` through the connection, refused first when it lacks
    /// [`ListPullRequestFiles::SCOPES`].
    pub async fn github_list_pull_request_files(
        &self,
        args: ActionArgs<'_, ListPullRequestFiles>,
    ) -> UseCaseResult<Vec<GithubPullRequestFile>> {
        self.run("github_list_pull_request_files", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_files",
                required_scopes: ListPullRequestFiles::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_pull_request_files(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
