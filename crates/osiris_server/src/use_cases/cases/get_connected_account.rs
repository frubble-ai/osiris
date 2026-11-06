//! One connection.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::ConnectedAccount;

pub struct GetConnectedAccountArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn get_connected_account(
        &self,
        args: GetConnectedAccountArgs<'_>,
    ) -> UseCaseResult<ConnectedAccount> {
        self.run("get_connected_account", async {
            self.context().db.connected_account(args.id).await
        })
        .await
    }
}
