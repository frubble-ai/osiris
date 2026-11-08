//! `github.delete_label`: a label deleted from a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::DeleteLabel, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_label` through the connection, refused first when it lacks
    /// [`DeleteLabel::SCOPES`].
    pub async fn github_delete_label(
        &self,
        args: GithubActionArgs<'_, DeleteLabel>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_label", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_label",
                required_scopes: DeleteLabel::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
