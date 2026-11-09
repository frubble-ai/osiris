//! `github.get_ref`: a branch's or a tag's git reference.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::GetRef, types::GithubGitRef};

impl UseCases {
    /// Runs `github.get_ref` through the connection, refused first when it lacks
    /// [`GetRef::SCOPES`].
    pub async fn github_get_ref(
        &self,
        args: ActionArgs<'_, GetRef>,
    ) -> UseCaseResult<GithubGitRef> {
        self.run("github_get_ref", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_ref",
                required_scopes: GetRef::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider.get_ref(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
