//! Cancels a connection's previous refresh task once a newer one replaces it.

use super::RunningTask;
use crate::core::Core;

impl Core {
    /// Cancels the connection's previous refresh task, unless it is the task running now (the
    /// scheduler finishes that one when the worker answers).
    pub(crate) async fn cancel_previous(
        &self,
        previous: Option<&str>,
        running: Option<RunningTask<'_>>,
    ) {
        if let Some(previous) = previous
            && running.is_none_or(|task| task.name != previous)
        {
            self.cancel_quietly(previous).await;
        }
    }
}
