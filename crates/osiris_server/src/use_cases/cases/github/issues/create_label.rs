//! `github.create_label`: a new label in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::CreateLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.create_label` through the connection, refused first when it lacks
    /// [`CreateLabel::SCOPES`].
    pub async fn github_create_label(
        &self,
        args: GithubActionArgs<'_, CreateLabel>,
    ) -> UseCaseResult<GithubLabel> {
        self.run("github_create_label", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_label",
                required_scopes: CreateLabel::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
