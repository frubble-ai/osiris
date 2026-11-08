//! `github.list_repository_tags`: a repository's tags, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListRepositoryTags, types::GithubTag};

impl UseCases {
    /// Runs `github.list_repository_tags` through the connection, refused first when it lacks
    /// [`ListRepositoryTags::SCOPES`].
    pub async fn github_list_repository_tags(
        &self,
        args: GithubActionArgs<'_, ListRepositoryTags>,
    ) -> UseCaseResult<Vec<GithubTag>> {
        self.run("github_list_repository_tags", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_tags",
                required_scopes: ListRepositoryTags::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_tags(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
