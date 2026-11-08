//! `github.list_assignees`: the people issues in a repository can be assigned to, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListAssignees, types::GithubUser};

impl UseCases {
    /// Runs `github.list_assignees` through the connection, refused first when it lacks
    /// [`ListAssignees::SCOPES`].
    pub async fn github_list_assignees(
        &self,
        args: GithubActionArgs<'_, ListAssignees>,
    ) -> UseCaseResult<Vec<GithubUser>> {
        self.run("github_list_assignees", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_assignees",
                required_scopes: ListAssignees::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_assignees(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
