//! `github.list_check_runs_for_ref`: the check runs on a commit, a branch or a tag, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::github::{Github, checks::ListCheckRunsForRef, types::GithubCheckRun};

impl UseCases {
    /// Runs `github.list_check_runs_for_ref` through the connection, refused first when it lacks
    /// [`ListCheckRunsForRef::SCOPES`].
    pub async fn github_list_check_runs_for_ref(
        &self,
        args: ActionArgs<'_, ListCheckRunsForRef>,
    ) -> UseCaseResult<Vec<GithubCheckRun>> {
        self.run("github_list_check_runs_for_ref", async {
            let ctx = self.context();
            let provider = provider_of::<Github>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_check_runs_for_ref",
                required_scopes: ListCheckRunsForRef::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_check_runs_for_ref(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
