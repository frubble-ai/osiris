//! A repository's topics.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A repository's topics, the lowercase names it is tagged with.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubTopics {
    pub names: Vec<String>,
}
