//! `github.list_issue_timeline_events`: everything on an issue's timeline, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, issues::ListIssueTimelineEvents, types::GithubTimelineEvent,
};

impl UseCases {
    /// Runs `github.list_issue_timeline_events` through the connection, refused first when it lacks
    /// [`ListIssueTimelineEvents::SCOPES`].
    pub async fn github_list_issue_timeline_events(
        &self,
        args: ActionArgs<'_, ListIssueTimelineEvents>,
    ) -> UseCaseResult<Vec<GithubTimelineEvent>> {
        self.run("github_list_issue_timeline_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_timeline_events",
                required_scopes: ListIssueTimelineEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_issue_timeline_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
