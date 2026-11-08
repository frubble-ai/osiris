//! `github.delete_ref`: a branch's or a tag's git reference deleted.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{repositories::DeleteRef, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_ref` through the connection, refused first when it lacks
    /// [`DeleteRef::SCOPES`].
    pub async fn github_delete_ref(
        &self,
        args: GithubActionArgs<'_, DeleteRef>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_ref", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_ref",
                required_scopes: DeleteRef::SCOPES,
                call: move |token: Secret| async move {
                    github.delete_ref(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
