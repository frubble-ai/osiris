//! `github.get_ref`: a branch's or a tag's git reference.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::GetRef, types::GithubGitRef};

impl UseCases {
    /// Runs `github.get_ref` through the connection, refused first when it lacks
    /// [`GetRef::SCOPES`].
    pub async fn github_get_ref(
        &self,
        args: GithubActionArgs<'_, GetRef>,
    ) -> UseCaseResult<GithubGitRef> {
        self.run("github_get_ref", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_ref",
                required_scopes: GetRef::SCOPES,
                call: move |token: Secret| async move {
                    github.get_ref(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
