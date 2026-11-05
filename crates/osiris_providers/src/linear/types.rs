//! Linear's reads, in our terms.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A Linear team the connected user can see.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct LinearTeam {
    pub id: String,
    pub name: String,
}
