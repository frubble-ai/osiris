//! `github.get_label`: a repository's label, by name.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::GetLabel, types::GithubLabel};

impl UseCases {
    /// Runs `github.get_label` through the connection, refused first when it lacks
    /// [`GetLabel::SCOPES`].
    pub async fn github_get_label(
        &self,
        args: ActionArgs<'_, GetLabel>,
    ) -> UseCaseResult<GithubLabel> {
        self.run("github_get_label", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_label",
                required_scopes: GetLabel::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider.get_label(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
