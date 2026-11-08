//! `github.get_repository_readme`: a repository's README, in base64 and decoded when it is text.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::GetRepositoryReadme, types::GithubFileContent};

impl UseCases {
    /// Runs `github.get_repository_readme` through the connection, refused first when it lacks
    /// [`GetRepositoryReadme::SCOPES`].
    pub async fn github_get_repository_readme(
        &self,
        args: GithubActionArgs<'_, GetRepositoryReadme>,
    ) -> UseCaseResult<GithubFileContent> {
        self.run("github_get_repository_readme", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_repository_readme",
                required_scopes: GetRepositoryReadme::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_repository_readme(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
