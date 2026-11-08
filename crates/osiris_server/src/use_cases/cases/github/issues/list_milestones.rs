//! `github.list_milestones`: a repository's milestones, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListMilestones, types::GithubMilestone};

impl UseCases {
    /// Runs `github.list_milestones` through the connection, refused first when it lacks
    /// [`ListMilestones::SCOPES`].
    pub async fn github_list_milestones(
        &self,
        args: GithubActionArgs<'_, ListMilestones>,
    ) -> UseCaseResult<Vec<GithubMilestone>> {
        self.run("github_list_milestones", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_milestones",
                required_scopes: ListMilestones::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_milestones(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
