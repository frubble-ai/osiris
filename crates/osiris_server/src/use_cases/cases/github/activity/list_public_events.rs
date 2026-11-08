//! `github.list_public_events`: GitHub's global feed of public events, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListPublicEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_public_events` through the connection, refused first when it lacks
    /// [`ListPublicEvents::SCOPES`].
    pub async fn github_list_public_events(
        &self,
        args: GithubActionArgs<'_, ListPublicEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_public_events", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_public_events",
                required_scopes: ListPublicEvents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_public_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
