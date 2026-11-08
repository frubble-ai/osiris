//! `github.update_milestone`: a milestone's title, state, description or due date changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::UpdateMilestone, types::GithubMilestone};

impl UseCases {
    /// Runs `github.update_milestone` through the connection, refused first when it lacks
    /// [`UpdateMilestone::SCOPES`].
    pub async fn github_update_milestone(
        &self,
        args: GithubActionArgs<'_, UpdateMilestone>,
    ) -> UseCaseResult<GithubMilestone> {
        self.run("github_update_milestone", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_milestone",
                required_scopes: UpdateMilestone::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .update_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
