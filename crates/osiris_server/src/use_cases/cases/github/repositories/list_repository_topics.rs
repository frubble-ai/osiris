//! `github.list_repository_topics`: a repository's topics.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::ListRepositoryTopics, types::GithubTopics};

impl UseCases {
    /// Runs `github.list_repository_topics` through the connection, refused first when it lacks
    /// [`ListRepositoryTopics::SCOPES`].
    pub async fn github_list_repository_topics(
        &self,
        args: ActionArgs<'_, ListRepositoryTopics>,
    ) -> UseCaseResult<GithubTopics> {
        self.run("github_list_repository_topics", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_topics",
                required_scopes: ListRepositoryTopics::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_topics(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
