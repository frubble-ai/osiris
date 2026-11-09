//! `github.list_issue_events`: what happened to an issue, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ListIssueEvents, types::GithubIssueEvent};

impl UseCases {
    /// Runs `github.list_issue_events` through the connection, refused first when it lacks
    /// [`ListIssueEvents::SCOPES`].
    pub async fn github_list_issue_events(
        &self,
        args: ActionArgs<'_, ListIssueEvents>,
    ) -> UseCaseResult<Vec<GithubIssueEvent>> {
        self.run("github_list_issue_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_events",
                required_scopes: ListIssueEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_issue_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
