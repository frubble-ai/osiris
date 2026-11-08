//! `github.get_milestone`: a milestone, by number.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::GetMilestone, types::GithubMilestone};

impl UseCases {
    /// Runs `github.get_milestone` through the connection, refused first when it lacks
    /// [`GetMilestone::SCOPES`].
    pub async fn github_get_milestone(
        &self,
        args: GithubActionArgs<'_, GetMilestone>,
    ) -> UseCaseResult<GithubMilestone> {
        self.run("github_get_milestone", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_milestone",
                required_scopes: GetMilestone::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
