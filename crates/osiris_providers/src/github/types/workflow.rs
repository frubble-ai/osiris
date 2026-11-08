//! A GitHub Actions workflow.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A GitHub Actions workflow.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubWorkflow {
    pub id: String,
    pub name: String,
    pub path: String,
    pub state: String,
    pub url: String,
}
