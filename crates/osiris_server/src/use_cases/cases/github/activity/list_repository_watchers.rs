//! `github.list_repository_watchers`: the users watching a repository, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, activity::ListRepositoryWatchers, types::GithubUser};

impl UseCases {
    /// Runs `github.list_repository_watchers` through the connection, refused first when it lacks
    /// [`ListRepositoryWatchers::SCOPES`].
    pub async fn github_list_repository_watchers(
        &self,
        args: ActionArgs<'_, ListRepositoryWatchers>,
    ) -> UseCaseResult<Vec<GithubUser>> {
        self.run("github_list_repository_watchers", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_watchers",
                required_scopes: ListRepositoryWatchers::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_watchers(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
