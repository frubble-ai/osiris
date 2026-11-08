//! `github.list_repository_watchers`: the users watching a repository, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListRepositoryWatchers, types::GithubUser};

impl UseCases {
    /// Runs `github.list_repository_watchers` through the connection, refused first when it lacks
    /// [`ListRepositoryWatchers::SCOPES`].
    pub async fn github_list_repository_watchers(
        &self,
        args: GithubActionArgs<'_, ListRepositoryWatchers>,
    ) -> UseCaseResult<Vec<GithubUser>> {
        self.run("github_list_repository_watchers", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_watchers",
                required_scopes: ListRepositoryWatchers::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_watchers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
