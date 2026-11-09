//! `github.list_milestones`: a repository's milestones, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ListMilestones, types::GithubMilestone};

impl UseCases {
    /// Runs `github.list_milestones` through the connection, refused first when it lacks
    /// [`ListMilestones::SCOPES`].
    pub async fn github_list_milestones(
        &self,
        args: ActionArgs<'_, ListMilestones>,
    ) -> UseCaseResult<Vec<GithubMilestone>> {
        self.run("github_list_milestones", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_milestones",
                required_scopes: ListMilestones::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_milestones(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
