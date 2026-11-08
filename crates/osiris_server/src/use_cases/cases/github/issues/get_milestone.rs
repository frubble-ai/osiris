//! `github.get_milestone`: a milestone, by number.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::GetMilestone, types::GithubMilestone};

impl UseCases {
    /// Runs `github.get_milestone` through the connection, refused first when it lacks
    /// [`GetMilestone::SCOPES`].
    pub async fn github_get_milestone(
        &self,
        args: ActionArgs<'_, GetMilestone>,
    ) -> UseCaseResult<GithubMilestone> {
        self.run("github_get_milestone", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_milestone",
                required_scopes: GetMilestone::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .get_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
