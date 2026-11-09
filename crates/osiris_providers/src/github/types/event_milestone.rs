//! The milestone an issue event is about.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The milestone a `milestoned` or `demilestoned` event is about.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubEventMilestone {
    pub title: String,
}
