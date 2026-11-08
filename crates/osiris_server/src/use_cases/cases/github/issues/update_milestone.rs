//! `github.update_milestone`: a milestone's title, state, description or due date changed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::UpdateMilestone, types::GithubMilestone};

impl UseCases {
    /// Runs `github.update_milestone` through the connection, refused first when it lacks
    /// [`UpdateMilestone::SCOPES`].
    pub async fn github_update_milestone(
        &self,
        args: ActionArgs<'_, UpdateMilestone>,
    ) -> UseCaseResult<GithubMilestone> {
        self.run("github_update_milestone", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_milestone",
                required_scopes: UpdateMilestone::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
