//! Keep a keyed request's first answer, to replay.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use serde_json::Value;

impl Core {
    pub async fn finish_idempotent(&self, key: &str, status: u16, body: &Value) -> CoreResult<()> {
        self.store
            .finish_idempotent(key, status, body)
            .await
            .switch()
    }
}
