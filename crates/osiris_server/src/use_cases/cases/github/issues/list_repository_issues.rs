//! `github.list_repository_issues`: a repository's issues, a page at a time, without the pull requests GitHub lists among them.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{issues::ListRepositoryIssues, types::GithubIssuePage};

impl UseCases {
    /// Runs `github.list_repository_issues` through the connection, refused first when it lacks
    /// [`ListRepositoryIssues::SCOPES`].
    pub async fn github_list_repository_issues(
        &self,
        args: GithubActionArgs<'_, ListRepositoryIssues>,
    ) -> UseCaseResult<GithubIssuePage> {
        self.run("github_list_repository_issues", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_repository_issues",
                required_scopes: ListRepositoryIssues::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_repository_issues(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
