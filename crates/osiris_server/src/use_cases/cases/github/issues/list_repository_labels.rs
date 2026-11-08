//! `github.list_repository_labels`: the labels a repository has, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListRepositoryLabels, types::GithubLabel};

impl UseCases {
    /// Runs `github.list_repository_labels` through the connection, refused first when it lacks
    /// [`ListRepositoryLabels::SCOPES`].
    pub async fn github_list_repository_labels(
        &self,
        args: GithubActionArgs<'_, ListRepositoryLabels>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_list_repository_labels", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_labels",
                required_scopes: ListRepositoryLabels::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
