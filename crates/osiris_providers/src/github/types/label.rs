//! A label.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A label.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    /// Whether it is one of the labels GitHub gives every new repository.
    pub default: bool,
}
