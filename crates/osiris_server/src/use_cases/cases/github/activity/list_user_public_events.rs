//! `github.list_user_public_events`: the public events a user performed, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::ListUserPublicEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_user_public_events` through the connection, refused first when it lacks
    /// [`ListUserPublicEvents::SCOPES`].
    pub async fn github_list_user_public_events(
        &self,
        args: ActionArgs<'_, ListUserPublicEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_user_public_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_user_public_events",
                required_scopes: ListUserPublicEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_user_public_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
