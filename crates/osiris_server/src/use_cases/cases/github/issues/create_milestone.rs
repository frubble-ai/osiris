//! `github.create_milestone`: a new milestone in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::CreateMilestone, types::GithubMilestone};

impl UseCases {
    /// Runs `github.create_milestone` through the connection, refused first when it lacks
    /// [`CreateMilestone::SCOPES`].
    pub async fn github_create_milestone(
        &self,
        args: GithubActionArgs<'_, CreateMilestone>,
    ) -> UseCaseResult<GithubMilestone> {
        self.run("github_create_milestone", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_milestone",
                required_scopes: CreateMilestone::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .create_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
