//! `github.list_authenticated_user_events`: the events a user performed, private ones too when the user is the connection's, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListAuthenticatedUserEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_authenticated_user_events` through the connection, refused first when it lacks
    /// [`ListAuthenticatedUserEvents::SCOPES`].
    pub async fn github_list_authenticated_user_events(
        &self,
        args: GithubActionArgs<'_, ListAuthenticatedUserEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_authenticated_user_events", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_authenticated_user_events",
                required_scopes: ListAuthenticatedUserEvents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_authenticated_user_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
