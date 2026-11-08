//! `github.create_ref`: a new git reference pointing at a commit.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::CreateRef, types::GithubGitRef};

impl UseCases {
    /// Runs `github.create_ref` through the connection, refused first when it lacks
    /// [`CreateRef::SCOPES`].
    pub async fn github_create_ref(
        &self,
        args: ActionArgs<'_, CreateRef>,
    ) -> UseCaseResult<GithubGitRef> {
        self.run("github_create_ref", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.create_ref",
                required_scopes: CreateRef::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .create_ref(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
