//! One connection.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::ConnectedAccount;

impl Core {
    pub async fn connected_account(&self, id: &str) -> CoreResult<ConnectedAccount> {
        self.store.connected_account(id).await.switch()
    }
}
