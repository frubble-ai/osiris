//! A workflow job's logs, as plain text: their end, when they are longer than osiris answers.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A workflow job's logs, as plain text.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubJobLogs {
    /// The log, or its last `[providers.listing] max_log_bytes` when it is longer.
    pub logs: String,
    /// Whether the log was longer, so only its end is here.
    pub truncated: bool,
}
