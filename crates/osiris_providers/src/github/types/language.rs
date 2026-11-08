//! A language a repository is written in, by bytes of code.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A language a repository is written in, by bytes of code.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubLanguage {
    pub name: String,
    pub bytes: i64,
}
