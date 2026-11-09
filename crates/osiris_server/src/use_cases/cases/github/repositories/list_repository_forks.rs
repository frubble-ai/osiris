//! `github.list_repository_forks`: a repository's forks, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{
    Github, repositories::ListRepositoryForks, types::GithubRepository,
};

impl UseCases {
    /// Runs `github.list_repository_forks` through the connection, refused first when it lacks
    /// [`ListRepositoryForks::SCOPES`].
    pub async fn github_list_repository_forks(
        &self,
        args: ActionArgs<'_, ListRepositoryForks>,
    ) -> UseCaseResult<Vec<GithubRepository>> {
        self.run("github_list_repository_forks", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_forks",
                required_scopes: ListRepositoryForks::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_forks(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
