//! `github.list_installations`: where the GitHub App is installed, of what the connected user can see.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, installations::ListInstallations, types::GithubInstallation,
};

impl UseCases {
    /// Runs `github.list_installations` through the connection, refused first when it lacks
    /// [`ListInstallations::SCOPES`].
    /// Only as a user: an installation connection is refused before the call.
    pub async fn github_list_installations(
        &self,
        args: ActionArgs<'_, ListInstallations>,
    ) -> UseCaseResult<Vec<GithubInstallation>> {
        self.run("github_list_installations", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_installations",
                required_scopes: ListInstallations::SCOPES,
                user_only: true,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_installations(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
