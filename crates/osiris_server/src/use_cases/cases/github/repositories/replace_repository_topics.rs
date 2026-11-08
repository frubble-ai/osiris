//! `github.replace_repository_topics`: a repository's topics replaced by a new set.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::ReplaceRepositoryTopics, types::GithubTopics,
};

impl UseCases {
    /// Runs `github.replace_repository_topics` through the connection, refused first when it lacks
    /// [`ReplaceRepositoryTopics::SCOPES`].
    pub async fn github_replace_repository_topics(
        &self,
        args: ActionArgs<'_, ReplaceRepositoryTopics>,
    ) -> UseCaseResult<GithubTopics> {
        self.run("github_replace_repository_topics", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.replace_repository_topics",
                required_scopes: ReplaceRepositoryTopics::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .replace_repository_topics(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
