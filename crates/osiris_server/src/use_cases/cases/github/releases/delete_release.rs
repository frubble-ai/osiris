//! `github.delete_release`: a release deleted, by its id. Its tag stays.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::DeleteRelease, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_release` through the connection, refused first when it lacks
    /// [`DeleteRelease::SCOPES`].
    pub async fn github_delete_release(
        &self,
        args: GithubActionArgs<'_, DeleteRelease>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_release", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_release",
                required_scopes: DeleteRelease::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
