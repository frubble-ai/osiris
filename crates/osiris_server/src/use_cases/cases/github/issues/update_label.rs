//! `github.update_label`: a label renamed, recolored or described anew.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::UpdateLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.update_label` through the connection, refused first when it lacks
    /// [`UpdateLabel::SCOPES`].
    pub async fn github_update_label(
        &self,
        args: ActionArgs<'_, UpdateLabel>,
    ) -> UseCaseResult<GithubLabel> {
        self.run("github_update_label", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_label",
                required_scopes: UpdateLabel::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_label(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
