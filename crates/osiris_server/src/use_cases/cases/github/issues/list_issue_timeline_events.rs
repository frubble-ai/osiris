//! `github.list_issue_timeline_events`: everything on an issue's timeline, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListIssueTimelineEvents, types::GithubTimelineEvent};

impl UseCases {
    /// Runs `github.list_issue_timeline_events` through the connection, refused first when it lacks
    /// [`ListIssueTimelineEvents::SCOPES`].
    pub async fn github_list_issue_timeline_events(
        &self,
        args: GithubActionArgs<'_, ListIssueTimelineEvents>,
    ) -> UseCaseResult<Vec<GithubTimelineEvent>> {
        self.run("github_list_issue_timeline_events", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_timeline_events",
                required_scopes: ListIssueTimelineEvents::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_issue_timeline_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
