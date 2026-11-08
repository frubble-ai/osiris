//! `github.list_repository_issue_events`: what happened to every issue in a repository, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, issues::ListRepositoryIssueEvents, types::GithubIssueEvent,
};

impl UseCases {
    /// Runs `github.list_repository_issue_events` through the connection, refused first when it lacks
    /// [`ListRepositoryIssueEvents::SCOPES`].
    pub async fn github_list_repository_issue_events(
        &self,
        args: ActionArgs<'_, ListRepositoryIssueEvents>,
    ) -> UseCaseResult<Vec<GithubIssueEvent>> {
        self.run("github_list_repository_issue_events", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_issue_events",
                required_scopes: ListRepositoryIssueEvents::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_issue_events(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
