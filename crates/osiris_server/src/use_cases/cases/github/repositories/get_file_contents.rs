//! `github.get_file_contents`: a file of a repository, in base64 and decoded when it is text.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::GetFileContents, types::GithubFileContent};

impl UseCases {
    /// Runs `github.get_file_contents` through the connection, refused first when it lacks
    /// [`GetFileContents::SCOPES`].
    pub async fn github_get_file_contents(
        &self,
        args: ActionArgs<'_, GetFileContents>,
    ) -> UseCaseResult<GithubFileContent> {
        self.run("github_get_file_contents", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_file_contents",
                required_scopes: GetFileContents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_file_contents(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
