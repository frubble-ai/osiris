//! `github.update_label`: a label renamed, recolored or described anew.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::UpdateLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.update_label` through the connection, refused first when it lacks
    /// [`UpdateLabel::SCOPES`].
    pub async fn github_update_label(
        &self,
        args: GithubActionArgs<'_, UpdateLabel>,
    ) -> UseCaseResult<GithubLabel> {
        self.run("github_update_label", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_label",
                required_scopes: UpdateLabel::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
