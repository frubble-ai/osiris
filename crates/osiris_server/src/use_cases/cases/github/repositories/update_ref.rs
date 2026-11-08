//! `github.update_ref`: a branch's or a tag's git reference pointed at another commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::UpdateRef, types::GithubGitRef};

impl UseCases {
    /// Runs `github.update_ref` through the connection, refused first when it lacks
    /// [`UpdateRef::SCOPES`].
    pub async fn github_update_ref(
        &self,
        args: GithubActionArgs<'_, UpdateRef>,
    ) -> UseCaseResult<GithubGitRef> {
        self.run("github_update_ref", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_ref",
                required_scopes: UpdateRef::SCOPES,
                call: move |token: Secret| async move {
                    github.update_ref(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
