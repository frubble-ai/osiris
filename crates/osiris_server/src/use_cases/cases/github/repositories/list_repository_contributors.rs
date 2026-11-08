//! `github.list_repository_contributors`: who contributed to a repository, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    repositories::ListRepositoryContributors, types::GithubContributor,
};

impl UseCases {
    /// Runs `github.list_repository_contributors` through the connection, refused first when it lacks
    /// [`ListRepositoryContributors::SCOPES`].
    pub async fn github_list_repository_contributors(
        &self,
        args: GithubActionArgs<'_, ListRepositoryContributors>,
    ) -> UseCaseResult<Vec<GithubContributor>> {
        self.run("github_list_repository_contributors", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_contributors",
                required_scopes: ListRepositoryContributors::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_contributors(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
