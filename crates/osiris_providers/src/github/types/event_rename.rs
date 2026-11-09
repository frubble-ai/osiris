//! An issue's title before and after a `renamed` event.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An issue's title before and after a `renamed` event.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubEventRename {
    pub from: String,
    pub to: String,
}
