//! `github.search_labels`: a repository's labels whose name or description match a search, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, search::SearchLabels, types::GithubLabelSearchResults};

impl UseCases {
    /// Runs `github.search_labels` through the connection, refused first when it lacks
    /// [`SearchLabels::SCOPES`].
    pub async fn github_search_labels(
        &self,
        args: ActionArgs<'_, SearchLabels>,
    ) -> UseCaseResult<GithubLabelSearchResults> {
        self.run("github_search_labels", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.search_labels",
                required_scopes: SearchLabels::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .search_labels(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
