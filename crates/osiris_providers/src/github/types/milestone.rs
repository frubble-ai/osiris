//! A milestone.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A milestone.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubMilestone {
    pub id: String,
    pub number: i64,
    pub title: String,
    pub state: String,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    /// When it was closed; none while it is open.
    pub closed_at: Option<String>,
}
