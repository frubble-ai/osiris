//! `github.list_repository_topics`: a repository's topics.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListRepositoryTopics, types::GithubTopics};

impl UseCases {
    /// Runs `github.list_repository_topics` through the connection, refused first when it lacks
    /// [`ListRepositoryTopics::SCOPES`].
    pub async fn github_list_repository_topics(
        &self,
        args: GithubActionArgs<'_, ListRepositoryTopics>,
    ) -> UseCaseResult<GithubTopics> {
        self.run("github_list_repository_topics", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_topics",
                required_scopes: ListRepositoryTopics::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_topics(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
