//! `github.list_repository_events`: a repository's recent events, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::ListRepositoryEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_repository_events` through the connection, refused first when it lacks
    /// [`ListRepositoryEvents::SCOPES`].
    pub async fn github_list_repository_events(
        &self,
        args: ActionArgs<'_, ListRepositoryEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_repository_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_events",
                required_scopes: ListRepositoryEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
