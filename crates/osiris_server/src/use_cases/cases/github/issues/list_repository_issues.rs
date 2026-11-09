//! `github.list_repository_issues`: a repository's issues, a page at a time, without the pull requests GitHub lists among them.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, issues::ListRepositoryIssues, types::GithubIssuePage};

impl UseCases {
    /// Runs `github.list_repository_issues` through the connection, refused first when it lacks
    /// [`ListRepositoryIssues::SCOPES`].
    pub async fn github_list_repository_issues(
        &self,
        args: ActionArgs<'_, ListRepositoryIssues>,
    ) -> UseCaseResult<GithubIssuePage> {
        self.run("github_list_repository_issues", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_issues",
                required_scopes: ListRepositoryIssues::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_repository_issues(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
