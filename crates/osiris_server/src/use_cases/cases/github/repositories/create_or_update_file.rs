//! `github.create_or_update_file`: a file of a repository written, in a commit of its own.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::CreateOrUpdateFile, types::GithubContentChange};

impl UseCases {
    /// Runs `github.create_or_update_file` through the connection, refused first when it lacks
    /// [`CreateOrUpdateFile::SCOPES`].
    pub async fn github_create_or_update_file(
        &self,
        args: GithubActionArgs<'_, CreateOrUpdateFile>,
    ) -> UseCaseResult<GithubContentChange> {
        self.run("github_create_or_update_file", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_or_update_file",
                required_scopes: CreateOrUpdateFile::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_or_update_file(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
