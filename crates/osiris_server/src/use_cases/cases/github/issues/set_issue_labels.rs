//! `github.set_issue_labels`: an issue's labels replaced with those given.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::SetIssueLabels, types::GithubLabel};

impl UseCases {
    /// Runs `github.set_issue_labels` through the connection, refused first when it lacks
    /// [`SetIssueLabels::SCOPES`].
    pub async fn github_set_issue_labels(
        &self,
        args: ActionArgs<'_, SetIssueLabels>,
    ) -> UseCaseResult<Vec<GithubLabel>> {
        self.run("github_set_issue_labels", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.set_issue_labels",
                required_scopes: SetIssueLabels::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .set_issue_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
