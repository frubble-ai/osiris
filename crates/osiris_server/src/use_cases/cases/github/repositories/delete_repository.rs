//! `github.delete_repository`: a repository deleted, by owner and name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::DeleteRepository, types::GithubAcknowledgement,
};

impl UseCases {
    /// Runs `github.delete_repository` through the connection, refused first when it lacks
    /// [`DeleteRepository::SCOPES`].
    pub async fn github_delete_repository(
        &self,
        args: ActionArgs<'_, DeleteRepository>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_repository", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_repository",
                required_scopes: DeleteRepository::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_repository(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
