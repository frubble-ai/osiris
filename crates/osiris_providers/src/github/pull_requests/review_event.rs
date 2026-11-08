//! What a review says of a pull request when it is submitted.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// What a review says of a pull request when it is submitted: it approves the changes, asks for
/// changes, or only comments.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewEvent {
    Approve,
    RequestChanges,
    Comment,
}
