//! `github.list_branches`: a repository's branches, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListBranches, types::GithubBranch};

impl UseCases {
    /// Runs `github.list_branches` through the connection, refused first when it lacks
    /// [`ListBranches::SCOPES`].
    pub async fn github_list_branches(
        &self,
        args: GithubActionArgs<'_, ListBranches>,
    ) -> UseCaseResult<Vec<GithubBranch>> {
        self.run("github_list_branches", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_branches",
                required_scopes: ListBranches::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_branches(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
