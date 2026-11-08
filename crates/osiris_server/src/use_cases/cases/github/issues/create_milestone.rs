//! `github.create_milestone`: a new milestone in a repository.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::CreateMilestone, types::GithubMilestone};

impl UseCases {
    /// Runs `github.create_milestone` through the connection, refused first when it lacks
    /// [`CreateMilestone::SCOPES`].
    pub async fn github_create_milestone(
        &self,
        args: ActionArgs<'_, CreateMilestone>,
    ) -> UseCaseResult<GithubMilestone> {
        self.run("github_create_milestone", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_milestone",
                required_scopes: CreateMilestone::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_milestone(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
