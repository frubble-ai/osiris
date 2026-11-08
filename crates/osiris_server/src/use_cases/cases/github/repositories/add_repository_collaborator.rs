//! `github.add_repository_collaborator`: an account added to a repository as a collaborator, or invited to be one.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    repositories::AddRepositoryCollaborator, types::GithubCollaboratorAddition,
};

impl UseCases {
    /// Runs `github.add_repository_collaborator` through the connection, refused first when it lacks
    /// [`AddRepositoryCollaborator::SCOPES`].
    pub async fn github_add_repository_collaborator(
        &self,
        args: GithubActionArgs<'_, AddRepositoryCollaborator>,
    ) -> UseCaseResult<GithubCollaboratorAddition> {
        self.run("github_add_repository_collaborator", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.add_repository_collaborator",
                required_scopes: AddRepositoryCollaborator::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .add_repository_collaborator(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
