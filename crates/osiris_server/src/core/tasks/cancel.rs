//! Cancels a task that is no longer needed. A failure is only logged: every task checks that its
//! job is still wanted before doing anything, so a task left behind simply does nothing.

use crate::core::Core;

impl Core {
    pub(crate) async fn cancel_quietly(&self, name: &str) {
        if let Err(error) = self.tasks.cancel(name).await {
            tracing::warn!(%error, task = name, "cancelling a task failed; it will do nothing when it runs");
        }
    }
}
