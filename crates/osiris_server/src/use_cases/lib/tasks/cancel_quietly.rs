//! Cancels a task that is no longer needed. A failure is only logged: every task checks that its
//! job is still wanted before doing anything, so a task left behind does nothing.

use super::Scheduling;

pub async fn cancel_quietly(scheduling: Scheduling<'_>, name: &str) {
    if let Err(error) = scheduling.tasks.cancel(name).await {
        tracing::warn!(%error, task = name, "cancelling a task failed; it will do nothing when it runs");
    }
}
