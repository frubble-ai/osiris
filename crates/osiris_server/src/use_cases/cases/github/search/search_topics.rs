//! `github.search_topics`: topics that match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{search::SearchTopics, types::GithubTopicSearchResults};

impl UseCases {
    /// Runs `github.search_topics` through the connection, refused first when it lacks
    /// [`SearchTopics::SCOPES`].
    pub async fn github_search_topics(
        &self,
        args: GithubActionArgs<'_, SearchTopics>,
    ) -> UseCaseResult<GithubTopicSearchResults> {
        self.run("github_search_topics", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_topics",
                required_scopes: SearchTopics::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .search_topics(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
