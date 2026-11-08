//! `linear.list_teams`: every team the connected user can see, archived ones left out.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::shared::{action_args::ActionArgs, provider_of::provider_of},
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_interfaces::ActingAs;
use osiris_providers::linear::{Linear, teams::ListTeams, types::LinearTeam};

impl UseCases {
    /// Runs `linear.list_teams` through the connection, refused first when it lacks
    /// [`ListTeams::SCOPES`].
    pub async fn linear_list_teams(
        &self,
        args: ActionArgs<'_, ListTeams>,
    ) -> UseCaseResult<Vec<LinearTeam>> {
        self.run("linear_list_teams", async {
            let ctx = self.context();
            let provider = provider_of::<Linear>(&ctx.providers)?;
            let (http, input) = (&ctx.http, args.input);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "linear",
                action: "linear.list_teams",
                required_scopes: ListTeams::SCOPES,
                user_only: false,
                call: move |token: Secret, _acting: ActingAs| async move {
                    provider
                        .list_teams(http, token.expose_secret(), input)
                        .await
                },
            })
            .await
        })
        .await
    }
}
