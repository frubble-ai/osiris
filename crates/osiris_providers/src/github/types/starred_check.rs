//! Whether the authenticated user has starred a repository.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Whether the authenticated user has starred a repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubStarredCheck {
    pub starred: bool,
}
