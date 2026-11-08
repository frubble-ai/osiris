//! `github.delete_milestone`: a milestone deleted from a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::DeleteMilestone, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_milestone` through the connection, refused first when it lacks
    /// [`DeleteMilestone::SCOPES`].
    pub async fn github_delete_milestone(
        &self,
        args: GithubActionArgs<'_, DeleteMilestone>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_milestone", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_milestone",
                required_scopes: DeleteMilestone::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .delete_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
