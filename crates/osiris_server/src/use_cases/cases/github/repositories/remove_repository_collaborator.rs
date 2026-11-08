//! `github.remove_repository_collaborator`: a collaborator removed from a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::RemoveRepositoryCollaborator, types::GithubAcknowledgement,
};

impl UseCases {
    /// Runs `github.remove_repository_collaborator` through the connection, refused first when it lacks
    /// [`RemoveRepositoryCollaborator::SCOPES`].
    pub async fn github_remove_repository_collaborator(
        &self,
        args: ActionArgs<'_, RemoveRepositoryCollaborator>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_remove_repository_collaborator", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_repository_collaborator",
                required_scopes: RemoveRepositoryCollaborator::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .remove_repository_collaborator(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
