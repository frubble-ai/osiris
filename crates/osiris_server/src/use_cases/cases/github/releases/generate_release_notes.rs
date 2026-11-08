//! `github.generate_release_notes`: the name and body GitHub writes for a release of a tag. Nothing is saved.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{releases::GenerateReleaseNotes, types::GithubReleaseNotes};

impl UseCases {
    /// Runs `github.generate_release_notes` through the connection, refused first when it lacks
    /// [`GenerateReleaseNotes::SCOPES`].
    pub async fn github_generate_release_notes(
        &self,
        args: GithubActionArgs<'_, GenerateReleaseNotes>,
    ) -> UseCaseResult<GithubReleaseNotes> {
        self.run("github_generate_release_notes", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.generate_release_notes",
                required_scopes: GenerateReleaseNotes::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .generate_release_notes(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
