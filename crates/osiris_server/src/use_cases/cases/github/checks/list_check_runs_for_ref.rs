//! `github.list_check_runs_for_ref`: the check runs on a commit, a branch or a tag, a page at a time.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        github::github_action_args::GithubActionArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::github::{checks::ListCheckRunsForRef, types::GithubCheckRun};

impl UseCases {
    /// Runs `github.list_check_runs_for_ref` through the connection, refused first when it lacks
    /// [`ListCheckRunsForRef::SCOPES`].
    pub async fn github_list_check_runs_for_ref(
        &self,
        args: GithubActionArgs<'_, ListCheckRunsForRef>,
    ) -> UseCaseResult<Vec<GithubCheckRun>> {
        self.run("github_list_check_runs_for_ref", async {
            let ctx = self.context();
            let (github, http, input) = (&ctx.providers.github, &ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "github",
                action: "github.list_check_runs_for_ref",
                required_scopes: ListCheckRunsForRef::SCOPES,
                call: move |token: Secret| async move {
                    github
                        .list_check_runs_for_ref(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
