//! The label an issue event is about.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The label a `labeled` or `unlabeled` event is about.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubEventLabel {
    pub name: String,
    pub color: String,
}
