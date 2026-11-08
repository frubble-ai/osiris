//! `github.list_public_events`: GitHub's global feed of public events, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::ListPublicEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_public_events` through the connection, refused first when it lacks
    /// [`ListPublicEvents::SCOPES`].
    pub async fn github_list_public_events(
        &self,
        args: ActionArgs<'_, ListPublicEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_public_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_public_events",
                required_scopes: ListPublicEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_public_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
