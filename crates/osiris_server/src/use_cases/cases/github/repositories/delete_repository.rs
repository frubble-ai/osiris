//! `github.delete_repository`: a repository deleted, by owner and name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::DeleteRepository, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_repository` through the connection, refused first when it lacks
    /// [`DeleteRepository::SCOPES`].
    pub async fn github_delete_repository(
        &self,
        args: GithubActionArgs<'_, DeleteRepository>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_repository", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_repository",
                required_scopes: DeleteRepository::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
