//! `github.list_repository_collaborators`: a repository's collaborators, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{
    repositories::ListRepositoryCollaborators, types::GithubCollaborator,
};

impl UseCases {
    /// Runs `github.list_repository_collaborators` through the connection, refused first when it lacks
    /// [`ListRepositoryCollaborators::SCOPES`].
    pub async fn github_list_repository_collaborators(
        &self,
        args: GithubActionArgs<'_, ListRepositoryCollaborators>,
    ) -> UseCaseResult<Vec<GithubCollaborator>> {
        self.run("github_list_repository_collaborators", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_collaborators",
                required_scopes: ListRepositoryCollaborators::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_collaborators(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
