//! Delete an app no connection uses.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;

impl Core {
    pub async fn delete_auth_config(&self, id: &str) -> CoreResult<()> {
        self.store.delete_auth_config(id).await.switch()
    }
}
