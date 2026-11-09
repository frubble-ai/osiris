//! `github.add_repository_collaborator`: an account added to a repository as a collaborator, or invited to be one.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::AddRepositoryCollaborator, types::GithubCollaboratorAddition,
};

impl UseCases {
    /// Runs `github.add_repository_collaborator` through the connection, refused first when it lacks
    /// [`AddRepositoryCollaborator::SCOPES`].
    pub async fn github_add_repository_collaborator(
        &self,
        args: ActionArgs<'_, AddRepositoryCollaborator>,
    ) -> UseCaseResult<GithubCollaboratorAddition> {
        self.run("github_add_repository_collaborator", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.add_repository_collaborator",
                required_scopes: AddRepositoryCollaborator::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .add_repository_collaborator(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
