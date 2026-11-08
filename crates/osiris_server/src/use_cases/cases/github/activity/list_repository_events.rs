//! `github.list_repository_events`: a repository's recent events, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{activity::ListRepositoryEvents, types::GithubEvent};

impl UseCases {
    /// Runs `github.list_repository_events` through the connection, refused first when it lacks
    /// [`ListRepositoryEvents::SCOPES`].
    pub async fn github_list_repository_events(
        &self,
        args: GithubActionArgs<'_, ListRepositoryEvents>,
    ) -> UseCaseResult<Vec<GithubEvent>> {
        self.run("github_list_repository_events", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_events",
                required_scopes: ListRepositoryEvents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
