//! `github.list_repository_languages`: the languages a repository is written in, by bytes of code.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::ListRepositoryLanguages, types::GithubLanguage,
};

impl UseCases {
    /// Runs `github.list_repository_languages` through the connection, refused first when it lacks
    /// [`ListRepositoryLanguages::SCOPES`].
    pub async fn github_list_repository_languages(
        &self,
        args: ActionArgs<'_, ListRepositoryLanguages>,
    ) -> UseCaseResult<Vec<GithubLanguage>> {
        self.run("github_list_repository_languages", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_languages",
                required_scopes: ListRepositoryLanguages::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_languages(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
