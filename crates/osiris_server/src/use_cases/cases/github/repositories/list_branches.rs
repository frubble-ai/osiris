//! `github.list_branches`: a repository's branches, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::ListBranches, types::GithubBranch};

impl UseCases {
    /// Runs `github.list_branches` through the connection, refused first when it lacks
    /// [`ListBranches::SCOPES`].
    pub async fn github_list_branches(
        &self,
        args: ActionArgs<'_, ListBranches>,
    ) -> UseCaseResult<Vec<GithubBranch>> {
        self.run("github_list_branches", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_branches",
                required_scopes: ListBranches::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_branches(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
