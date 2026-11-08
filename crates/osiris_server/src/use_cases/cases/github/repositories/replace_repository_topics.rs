//! `github.replace_repository_topics`: a repository's topics replaced by a new set.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ReplaceRepositoryTopics, types::GithubTopics};

impl UseCases {
    /// Runs `github.replace_repository_topics` through the connection, refused first when it lacks
    /// [`ReplaceRepositoryTopics::SCOPES`].
    pub async fn github_replace_repository_topics(
        &self,
        args: GithubActionArgs<'_, ReplaceRepositoryTopics>,
    ) -> UseCaseResult<GithubTopics> {
        self.run("github_replace_repository_topics", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.replace_repository_topics",
                required_scopes: ReplaceRepositoryTopics::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .replace_repository_topics(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
