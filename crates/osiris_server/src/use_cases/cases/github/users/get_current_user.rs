//! `github.get_current_user`: the authenticated user and their profile.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{types::GithubCurrentUser, users::GetCurrentUser};

impl UseCases {
    /// Runs `github.get_current_user` through the connection, refused first when it lacks
    /// [`GetCurrentUser::SCOPES`].
    pub async fn github_get_current_user(
        &self,
        args: GithubActionArgs<'_, GetCurrentUser>,
    ) -> UseCaseResult<GithubCurrentUser> {
        self.run("github_get_current_user", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.get_current_user",
                required_scopes: GetCurrentUser::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .get_current_user(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
