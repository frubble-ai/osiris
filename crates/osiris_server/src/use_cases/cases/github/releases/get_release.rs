//! `github.get_release`: a release, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::GetRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.get_release` through the connection, refused first when it lacks
    /// [`GetRelease::SCOPES`].
    pub async fn github_get_release(
        &self,
        args: GithubActionArgs<'_, GetRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_get_release", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_release",
                required_scopes: GetRelease::SCOPES,
                call: move |token: Secret| async move {
                    github.get_release(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
