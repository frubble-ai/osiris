//! The providers' apps, in batches.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::AuthConfig;
use osiris_storage::{Cursor, Page};

impl Core {
    pub async fn list_auth_configs(&self, cursor: &Cursor) -> CoreResult<Page<AuthConfig>> {
        self.store.list_auth_configs(cursor).await.switch()
    }
}
