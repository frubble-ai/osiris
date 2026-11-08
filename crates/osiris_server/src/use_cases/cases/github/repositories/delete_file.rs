//! `github.delete_file`: a file of a repository deleted, in a commit of its own.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::DeleteFile, types::GithubContentChange};

impl UseCases {
    /// Runs `github.delete_file` through the connection, refused first when it lacks
    /// [`DeleteFile::SCOPES`].
    pub async fn github_delete_file(
        &self,
        args: GithubActionArgs<'_, DeleteFile>,
    ) -> UseCaseResult<GithubContentChange> {
        self.run("github_delete_file", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_file",
                required_scopes: DeleteFile::SCOPES,
                call: move |token: Secret| async move {
                    github.delete_file(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
