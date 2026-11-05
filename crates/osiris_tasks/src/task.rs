//! A scheduled task: its name, when it should run, and the job it does.

use crate::TaskPayload;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    /// Built from the job the task does (see [`crate::names`]), so the same job always gets the
    /// same name.
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub run_at: OffsetDateTime,
    #[serde(flatten)]
    pub payload: TaskPayload,
}
