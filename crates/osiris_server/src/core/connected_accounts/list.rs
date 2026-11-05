//! Connections, filtered, in batches; `alias` finds what an attempt made.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::ConnectedAccount;
use osiris_storage::{Cursor, Page, connected_accounts::AccountFilter};

impl Core {
    pub async fn list_connected_accounts(
        &self,
        filter: &AccountFilter,
        cursor: &Cursor,
    ) -> CoreResult<Page<ConnectedAccount>> {
        self.store
            .list_connected_accounts(filter, cursor)
            .await
            .switch()
    }
}
