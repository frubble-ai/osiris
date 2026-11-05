//! The provider permanently rejects the connection's token, which means the grant no longer exists
//! there. The connection is marked as expired, a `connection.revoked` event is published, its
//! refresh task is cancelled, and the action answers that the user must reconnect.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::DomainError;

impl Core {
    pub(crate) async fn revoked<T>(&self, id: &str, reason: &str) -> CoreResult<T> {
        // The provider's own text stays in the log; the connection stores a fixed reason.
        tracing::warn!(connected_account_id = id, %reason, "the provider rejects the token; the grant is gone");
        let lock = self.store.lock_for_refresh(id).await.switch()?;
        let task = lock.account.refresh_task.clone();
        let event = lock
            .revoke("the provider no longer accepts the connection's token; reconnect")
            .await
            .switch()?;
        self.publish(&event).await;
        if let Some(task) = task {
            self.cancel_quietly(&task).await;
        }
        Err(DomainError::NotActive("expired").into())
    }
}
