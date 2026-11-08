//! `github.list_matching_refs`: every git reference that starts with a prefix.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::ListMatchingRefs, types::GithubGitRef};

impl UseCases {
    /// Runs `github.list_matching_refs` through the connection, refused first when it lacks
    /// [`ListMatchingRefs::SCOPES`].
    pub async fn github_list_matching_refs(
        &self,
        args: GithubActionArgs<'_, ListMatchingRefs>,
    ) -> UseCaseResult<Vec<GithubGitRef>> {
        self.run("github_list_matching_refs", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_matching_refs",
                required_scopes: ListMatchingRefs::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_matching_refs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
