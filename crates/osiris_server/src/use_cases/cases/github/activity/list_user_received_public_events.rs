//! `github.list_user_received_public_events`: the public events a user received from the accounts and repositories they follow and watch, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListUserReceivedPublicEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_user_received_public_events` through the connection, refused first when it lacks
    /// [`ListUserReceivedPublicEvents::SCOPES`].
    pub async fn github_list_user_received_public_events(
        &self,
        args: GithubActionArgs<'_, ListUserReceivedPublicEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_user_received_public_events", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_user_received_public_events",
                required_scopes: ListUserReceivedPublicEvents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_user_received_public_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
