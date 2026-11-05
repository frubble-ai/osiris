//! Slack's reads, in our terms.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A channel the connected user can see.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct SlackChannel {
    pub id: String,
    pub name: String,
    pub is_private: bool,
    /// Absent when Slack leaves it out.
    pub num_members: Option<u64>,
}
