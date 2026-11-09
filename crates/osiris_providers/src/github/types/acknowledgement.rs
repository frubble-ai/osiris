//! An action that answers nothing of its own: `ok` once GitHub has done it.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An action that answers nothing of its own: `ok` once GitHub has done it.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubAcknowledgement {
    pub ok: bool,
}
