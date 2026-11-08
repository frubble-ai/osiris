//! `github.generate_release_notes`: the name and body GitHub writes for a release of a tag. Nothing is saved.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::GenerateReleaseNotes, types::GithubReleaseNotes};

impl UseCases {
    /// Runs `github.generate_release_notes` through the connection, refused first when it lacks
    /// [`GenerateReleaseNotes::SCOPES`].
    pub async fn github_generate_release_notes(
        &self,
        args: ActionArgs<'_, GenerateReleaseNotes>,
    ) -> UseCaseResult<GithubReleaseNotes> {
        self.run("github_generate_release_notes", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.generate_release_notes",
                required_scopes: GenerateReleaseNotes::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .generate_release_notes(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
