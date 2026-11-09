//! `github.list_user_received_public_events`: the public events a user received from the accounts and repositories they follow and watch, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, activity::ListUserReceivedPublicEvents, types::GithubEvent,
};

impl UseCases {
    /// Runs `github.list_user_received_public_events` through the connection, refused first when it lacks
    /// [`ListUserReceivedPublicEvents::SCOPES`].
    pub async fn github_list_user_received_public_events(
        &self,
        args: ActionArgs<'_, ListUserReceivedPublicEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_user_received_public_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_user_received_public_events",
                required_scopes: ListUserReceivedPublicEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_user_received_public_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
