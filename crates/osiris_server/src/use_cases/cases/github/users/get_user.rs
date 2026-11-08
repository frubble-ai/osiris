//! `github.get_user`: a user's public profile, by login.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubPublicUser, users::GetUser};

impl UseCases {
    /// Runs `github.get_user` through the connection, refused first when it lacks
    /// [`GetUser::SCOPES`].
    pub async fn github_get_user(
        &self,
        args: GithubActionArgs<'_, GetUser>,
    ) -> UseCaseResult<GithubPublicUser> {
        self.run("github_get_user", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_user",
                required_scopes: GetUser::SCOPES,
                call: move |token: Secret| async move {
                    github.get_user(http, token.expose_secret(), input).await
                },
            })
            .await
        })
        .await
    }
}
