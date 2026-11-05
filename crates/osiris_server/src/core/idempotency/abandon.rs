//! A keyed request failed before answering: free its key for a retry.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;

impl Core {
    pub async fn abandon_idempotent(&self, key: &str) -> CoreResult<()> {
        self.store.abandon_idempotent(key).await.switch()
    }
}
