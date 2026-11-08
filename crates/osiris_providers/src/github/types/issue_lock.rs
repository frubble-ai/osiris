//! Whether an issue's conversation is locked, once GitHub has locked or unlocked it.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Whether an issue's conversation is locked, once GitHub has locked or unlocked it.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubIssueLock {
    pub locked: bool,
}
