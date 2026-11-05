//! Connections, filtered, a page at a time; `alias` finds what an attempt made.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::ConnectedAccount;
use osiris_storage::{Cursor, Page, connected_accounts::AccountFilter};

pub struct ListConnectedAccountsArgs<'a> {
    pub filter: &'a AccountFilter,
    pub cursor: &'a Cursor,
}

impl UseCases {
    pub async fn list_connected_accounts(
        &self,
        args: ListConnectedAccountsArgs<'_>,
    ) -> UseCaseResult<Page<ConnectedAccount>> {
        self.run("list_connected_accounts", async {
            self.context()
                .store
                .list_connected_accounts(args.filter, args.cursor)
                .await
                .switch()
        })
        .await
    }
}
