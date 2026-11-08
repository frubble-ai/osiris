//! The side of a diff a review comment is placed on.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The side of a diff a review comment is placed on: `LEFT` is the old version of the file, with
/// the lines removed, and `RIGHT` is the new version, with the lines added.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DiffSide {
    Left,
    Right,
}
