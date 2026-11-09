//! `github.list_authenticated_user_events`: the events a user performed, private ones too when the user is the connection's, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::ListAuthenticatedUserEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_authenticated_user_events` through the connection, refused first when it lacks
    /// [`ListAuthenticatedUserEvents::SCOPES`].
    pub async fn github_list_authenticated_user_events(
        &self,
        args: ActionArgs<'_, ListAuthenticatedUserEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_authenticated_user_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_authenticated_user_events",
                required_scopes: ListAuthenticatedUserEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_authenticated_user_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
