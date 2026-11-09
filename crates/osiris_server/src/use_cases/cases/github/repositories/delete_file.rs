//! `github.delete_file`: a file of a repository deleted, in a commit of its own.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::DeleteFile, types::GithubContentChange};

impl UseCases {
    /// Runs `github.delete_file` through the connection, refused first when it lacks
    /// [`DeleteFile::SCOPES`].
    pub async fn github_delete_file(
        &self,
        args: ActionArgs<'_, DeleteFile>,
    ) -> UseCaseResult<GithubContentChange> {
        self.run("github_delete_file", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_file",
                required_scopes: DeleteFile::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_file(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
