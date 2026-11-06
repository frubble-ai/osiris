//! An action's answer: what it read, as `data`.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An action's answer: what it read, as `data`.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ActionResult<T> {
    pub data: T,
}
