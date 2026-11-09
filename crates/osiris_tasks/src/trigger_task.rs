//! The payload of a `trigger.poll` task: check one trigger at the provider, for a trigger kind
//! that checks on a schedule (a watched pull request).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerTask {
    /// `tr_…`.
    pub trigger_id: String,
    pub connected_account_id: String,
    /// Which of the trigger's scheduled checks this is, from 1. It names the task, so a retry
    /// that schedules the next check again schedules the same one.
    pub check: i64,
}
