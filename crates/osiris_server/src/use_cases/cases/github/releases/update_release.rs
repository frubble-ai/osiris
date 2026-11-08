//! `github.update_release`: a release changed, by its id.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::UpdateRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.update_release` through the connection, refused first when it lacks
    /// [`UpdateRelease::SCOPES`].
    pub async fn github_update_release(
        &self,
        args: GithubActionArgs<'_, UpdateRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_update_release", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_release",
                required_scopes: UpdateRelease::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
