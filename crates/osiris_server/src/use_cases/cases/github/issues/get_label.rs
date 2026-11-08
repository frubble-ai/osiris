//! `github.get_label`: a repository's label, by name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::GetLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.get_label` through the connection, refused first when it lacks
    /// [`GetLabel::SCOPES`].
    pub async fn github_get_label(
        &self,
        args: GithubActionArgs<'_, GetLabel>,
    ) -> UseCaseResult<GithubLabel> {
        self.run("github_get_label", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_label",
                required_scopes: GetLabel::SCOPES,
                call: move |token: Secret| async move {
                    github.get_label(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
