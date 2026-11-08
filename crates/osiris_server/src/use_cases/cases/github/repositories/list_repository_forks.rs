//! `github.list_repository_forks`: a repository's forks, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListRepositoryForks, types::GithubRepository};

impl UseCases {
    /// Runs `github.list_repository_forks` through the connection, refused first when it lacks
    /// [`ListRepositoryForks::SCOPES`].
    pub async fn github_list_repository_forks(
        &self,
        args: GithubActionArgs<'_, ListRepositoryForks>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_repository_forks", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_forks",
                required_scopes: ListRepositoryForks::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_forks(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
