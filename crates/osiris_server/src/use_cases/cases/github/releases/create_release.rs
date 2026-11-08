//! `github.create_release`: a new release of a repository, from a tag.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::CreateRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.create_release` through the connection, refused first when it lacks
    /// [`CreateRelease::SCOPES`].
    pub async fn github_create_release(
        &self,
        args: GithubActionArgs<'_, CreateRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_create_release", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_release",
                required_scopes: CreateRelease::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
