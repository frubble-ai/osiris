//! `github.list_issue_labels`: the labels on an issue, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ListIssueLabels, types::GithubLabel};

impl UseCases {
    /// Runs `github.list_issue_labels` through the connection, refused first when it lacks
    /// [`ListIssueLabels::SCOPES`].
    pub async fn github_list_issue_labels(
        &self,
        args: ActionArgs<'_, ListIssueLabels>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_list_issue_labels", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_issue_labels",
                required_scopes: ListIssueLabels::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
