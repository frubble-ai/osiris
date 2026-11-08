//! `github.list_issue_events`: what happened to an issue, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListIssueEvents, types::GithubIssueEvent};

impl UseCases {
    /// Runs `github.list_issue_events` through the connection, refused first when it lacks
    /// [`ListIssueEvents::SCOPES`].
    pub async fn github_list_issue_events(
        &self,
        args: GithubActionArgs<'_, ListIssueEvents>,
    ) -> UseCaseResult<Vec<GithubIssueEvent>> {
        self.run("github_list_issue_events", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_events",
                required_scopes: ListIssueEvents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_issue_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
