//! `github.list_releases`: a repository's releases, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::ListReleases, types::GithubRelease};

impl UseCases {
    /// Runs `github.list_releases` through the connection, refused first when it lacks
    /// [`ListReleases::SCOPES`].
    pub async fn github_list_releases(
        &self,
        args: GithubActionArgs<'_, ListReleases>,
    ) -> UseCaseResult<Vec<GithubRelease>> {
        self.run("github_list_releases", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_releases",
                required_scopes: ListReleases::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_releases(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
