//! A team of an organization.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A team of an organization.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubTeam {
    pub id: String,
    pub name: String,
    pub slug: String,
}
