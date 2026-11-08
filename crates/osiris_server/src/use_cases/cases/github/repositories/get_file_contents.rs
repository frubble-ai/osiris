//! `github.get_file_contents`: a file of a repository, in base64 and decoded when it is text.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::GetFileContents, types::GithubFileContent};

impl UseCases {
    /// Runs `github.get_file_contents` through the connection, refused first when it lacks
    /// [`GetFileContents::SCOPES`].
    pub async fn github_get_file_contents(
        &self,
        args: GithubActionArgs<'_, GetFileContents>,
    ) -> UseCaseResult<GithubFileContent> {
        self.run("github_get_file_contents", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_file_contents",
                required_scopes: GetFileContents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_file_contents(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
