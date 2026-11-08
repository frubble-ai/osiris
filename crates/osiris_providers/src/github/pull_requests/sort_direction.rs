//! The order a list is sorted in.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The order a list is sorted in: ascending or descending.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    /// The order as GitHub names it in a query.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}
