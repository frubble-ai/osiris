//! `github.list_repository_stargazers`: the users who starred a repository, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListRepositoryStargazers, types::GithubUser};

impl UseCases {
    /// Runs `github.list_repository_stargazers` through the connection, refused first when it lacks
    /// [`ListRepositoryStargazers::SCOPES`].
    pub async fn github_list_repository_stargazers(
        &self,
        args: GithubActionArgs<'_, ListRepositoryStargazers>,
    ) -> UseCaseResult<Vec<GithubUser>> {
        self.run("github_list_repository_stargazers", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_stargazers",
                required_scopes: ListRepositoryStargazers::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_stargazers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
