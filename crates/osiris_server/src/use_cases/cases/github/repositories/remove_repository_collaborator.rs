//! `github.remove_repository_collaborator`: a collaborator removed from a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    repositories::RemoveRepositoryCollaborator, types::GithubAcknowledgement,
};

impl UseCases {
    /// Runs `github.remove_repository_collaborator` through the connection, refused first when it lacks
    /// [`RemoveRepositoryCollaborator::SCOPES`].
    pub async fn github_remove_repository_collaborator(
        &self,
        args: GithubActionArgs<'_, RemoveRepositoryCollaborator>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_remove_repository_collaborator", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.remove_repository_collaborator",
                required_scopes: RemoveRepositoryCollaborator::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .remove_repository_collaborator(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
