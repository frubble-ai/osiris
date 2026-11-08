//! `github.list_directory_contents`: the entries of a directory of a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::ListDirectoryContents, types::GithubContentEntry,
};

impl UseCases {
    /// Runs `github.list_directory_contents` through the connection, refused first when it lacks
    /// [`ListDirectoryContents::SCOPES`].
    pub async fn github_list_directory_contents(
        &self,
        args: ActionArgs<'_, ListDirectoryContents>,
    ) -> UseCaseResult<Vec<GithubContentEntry>> {
        self.run("github_list_directory_contents", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_directory_contents",
                required_scopes: ListDirectoryContents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_directory_contents(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
