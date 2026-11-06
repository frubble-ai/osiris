//! `linear.list_teams`: the Linear teams the connected user can see.

use crate::use_cases::{
    UseCaseResult, UseCases, cases::run_connected_account_action::RunConnectedAccountActionArgs,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_providers::linear::types::LinearTeam;

pub struct ListLinearTeamsArgs<'a> {
    /// The connection to read through.
    pub id: &'a str,
}

impl UseCases {
    pub async fn list_linear_teams(
        &self,
        args: ListLinearTeamsArgs<'_>,
    ) -> UseCaseResult<Vec<LinearTeam>> {
        self.run("list_linear_teams", async {
            let ctx = self.context();
            let (provider, http) = (&ctx.providers.linear, &ctx.http);
            self.run_connected_account_action(RunConnectedAccountActionArgs {
                id: args.id,
                provider: "linear",
                action: "linear.list_teams",
                call: move |token: Secret| async move {
                    provider.list_teams(http, token.expose_secret()).await
                },
            })
            .await
        })
        .await
    }
}
