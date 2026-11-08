//! `github.search_labels`: a repository's labels whose name or description match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{search::SearchLabels, types::GithubLabelSearchResults};

impl UseCases {
    /// Runs `github.search_labels` through the connection, refused first when it lacks
    /// [`SearchLabels::SCOPES`].
    pub async fn github_search_labels(
        &self,
        args: GithubActionArgs<'_, SearchLabels>,
    ) -> UseCaseResult<GithubLabelSearchResults> {
        self.run("github_search_labels", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_labels",
                required_scopes: SearchLabels::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .search_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
