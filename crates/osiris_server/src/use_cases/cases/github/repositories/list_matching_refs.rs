//! `github.list_matching_refs`: every git reference that starts with a prefix.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, repositories::ListMatchingRefs, types::GithubGitRef};

impl UseCases {
    /// Runs `github.list_matching_refs` through the connection, refused first when it lacks
    /// [`ListMatchingRefs::SCOPES`].
    pub async fn github_list_matching_refs(
        &self,
        args: ActionArgs<'_, ListMatchingRefs>,
    ) -> UseCaseResult<Vec<GithubGitRef>> {
        self.run("github_list_matching_refs", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_matching_refs",
                required_scopes: ListMatchingRefs::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_matching_refs(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
