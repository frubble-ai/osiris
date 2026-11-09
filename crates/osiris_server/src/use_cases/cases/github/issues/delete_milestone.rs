//! `github.delete_milestone`: a milestone deleted from a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::DeleteMilestone, types::GithubAcknowledgement};

impl UseCases {
    /// Runs `github.delete_milestone` through the connection, refused first when it lacks
    /// [`DeleteMilestone::SCOPES`].
    pub async fn github_delete_milestone(
        &self,
        args: ActionArgs<'_, DeleteMilestone>,
    ) -> UseCaseResult<GithubAcknowledgement> {
        self.run("github_delete_milestone", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.delete_milestone",
                required_scopes: DeleteMilestone::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .delete_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
