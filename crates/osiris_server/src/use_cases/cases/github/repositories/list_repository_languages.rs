//! `github.list_repository_languages`: the languages a repository is written in, by bytes of code.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListRepositoryLanguages, types::GithubLanguage};

impl UseCases {
    /// Runs `github.list_repository_languages` through the connection, refused first when it lacks
    /// [`ListRepositoryLanguages::SCOPES`].
    pub async fn github_list_repository_languages(
        &self,
        args: GithubActionArgs<'_, ListRepositoryLanguages>,
    ) -> UseCaseResult<Vec<GithubLanguage>> {
        self.run("github_list_repository_languages", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_languages",
                required_scopes: ListRepositoryLanguages::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_languages(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
