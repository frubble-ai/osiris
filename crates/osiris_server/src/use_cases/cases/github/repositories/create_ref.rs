//! `github.create_ref`: a new git reference pointing at a commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::CreateRef, types::GithubGitRef};

impl UseCases {
    /// Runs `github.create_ref` through the connection, refused first when it lacks
    /// [`CreateRef::SCOPES`].
    pub async fn github_create_ref(
        &self,
        args: GithubActionArgs<'_, CreateRef>,
    ) -> UseCaseResult<GithubGitRef> {
        self.run("github_create_ref", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_ref",
                required_scopes: CreateRef::SCOPES,
                call: move |token: Secret| async move {
                    github.create_ref(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
