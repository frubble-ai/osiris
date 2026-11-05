//! A keyed request arrives: run it, replay its first answer, or refuse it.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_storage::idempotency::Begun;

impl Core {
    pub async fn begin_idempotent(&self, key: &str, request_hash: &str) -> CoreResult<Begun> {
        self.store
            .begin_idempotent(
                key,
                request_hash,
                self.settings.idempotency.retention(),
                self.settings.idempotency.purge_batch,
            )
            .await
            .switch()
    }
}
