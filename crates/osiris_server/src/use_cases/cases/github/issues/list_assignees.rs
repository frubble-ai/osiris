//! `github.list_assignees`: the people issues in a repository can be assigned to, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ListAssignees, types::GithubUser};

impl UseCases {
    /// Runs `github.list_assignees` through the connection, refused first when it lacks
    /// [`ListAssignees::SCOPES`].
    pub async fn github_list_assignees(
        &self,
        args: ActionArgs<'_, ListAssignees>,
    ) -> UseCaseResult<Vec<GithubUser>> {
        self.run("github_list_assignees", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_assignees",
                required_scopes: ListAssignees::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_assignees(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
