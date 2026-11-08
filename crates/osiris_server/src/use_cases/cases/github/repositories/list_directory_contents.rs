//! `github.list_directory_contents`: the entries of a directory of a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListDirectoryContents, types::GithubContentEntry};

impl UseCases {
    /// Runs `github.list_directory_contents` through the connection, refused first when it lacks
    /// [`ListDirectoryContents::SCOPES`].
    pub async fn github_list_directory_contents(
        &self,
        args: GithubActionArgs<'_, ListDirectoryContents>,
    ) -> UseCaseResult<Vec<GithubContentEntry>> {
        self.run("github_list_directory_contents", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_directory_contents",
                required_scopes: ListDirectoryContents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_directory_contents(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
