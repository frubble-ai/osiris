//! Cancels a connection's previous refresh task once a newer one replaces it, unless it is the
//! task running now (the scheduler finishes that one when the worker answers).

use super::RunningTask;
use crate::use_cases::lib::tasks::{Scheduling, cancel_quietly::cancel_quietly};

pub async fn cancel_previous(
    scheduling: Scheduling<'_>,
    previous: Option<&str>,
    running: Option<RunningTask<'_>>,
) {
    if let Some(previous) = previous
        && running.is_none_or(|task| task.name != previous)
    {
        cancel_quietly(scheduling, previous).await;
    }
}
