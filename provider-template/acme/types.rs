//! Acme's reads, in our terms: what the API answers, with `ToSchema` for the spec.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct AcmeWidget {
    pub id: String,
    pub name: String,
}
