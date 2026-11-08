//! A workflow job's logs, as plain text.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A workflow job's logs, as plain text.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubJobLogs {
    /// The whole log, as GitHub keeps it once the job has finished.
    pub logs: String,
}
