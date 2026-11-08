//! `github.search_topics`: topics that match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, search::SearchTopics, types::GithubTopicSearchResults};

impl UseCases {
    /// Runs `github.search_topics` through the connection, refused first when it lacks
    /// [`SearchTopics::SCOPES`].
    pub async fn github_search_topics(
        &self,
        args: ActionArgs<'_, SearchTopics>,
    ) -> UseCaseResult<GithubTopicSearchResults> {
        self.run("github_search_topics", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_topics",
                required_scopes: SearchTopics::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_topics(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
