//! `github.list_pull_request_files`: the files a pull request changes, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{pull_requests::ListPullRequestFiles, types::GithubPullRequestFile};

impl UseCases {
    /// Runs `github.list_pull_request_files` through the connection, refused first when it lacks
    /// [`ListPullRequestFiles::SCOPES`].
    pub async fn github_list_pull_request_files(
        &self,
        args: GithubActionArgs<'_, ListPullRequestFiles>,
    ) -> UseCaseResult<Vec<GithubPullRequestFile>> {
        self.run("github_list_pull_request_files", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_pull_request_files",
                required_scopes: ListPullRequestFiles::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_pull_request_files(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
