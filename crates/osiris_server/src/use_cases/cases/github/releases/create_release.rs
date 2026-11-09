//! `github.create_release`: a new release of a repository, from a tag.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, releases::CreateRelease, types::GithubRelease};

impl UseCases {
    /// Runs `github.create_release` through the connection, refused first when it lacks
    /// [`CreateRelease::SCOPES`].
    pub async fn github_create_release(
        &self,
        args: ActionArgs<'_, CreateRelease>,
    ) -> UseCaseResult<GithubRelease> {
        self.run("github_create_release", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_release",
                required_scopes: CreateRelease::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_release(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
