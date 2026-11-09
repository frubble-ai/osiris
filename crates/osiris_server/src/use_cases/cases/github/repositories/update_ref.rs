//! `github.update_ref`: a branch's or a tag's git reference pointed at another commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::UpdateRef, types::GithubGitRef};

impl UseCases {
    /// Runs `github.update_ref` through the connection, refused first when it lacks
    /// [`UpdateRef::SCOPES`].
    pub async fn github_update_ref(
        &self,
        args: ActionArgs<'_, UpdateRef>,
    ) -> UseCaseResult<GithubGitRef> {
        self.run("github_update_ref", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.update_ref",
                required_scopes: UpdateRef::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .update_ref(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
