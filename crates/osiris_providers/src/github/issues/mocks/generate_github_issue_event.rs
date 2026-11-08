//! Something that happened to an issue, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use serde_json::{Value, json};

/// Event `id` of kind `event`, done by a generated actor in a generated commit.
pub fn generate_github_issue_event(id: u64, event: &str) -> Value {
    json!({
        "id": id,
        "event": event,
        "actor": generate_github_user(id + 1),
        "created_at": "2026-01-01T00:00:00Z",
        "commit_id": generate_sha(),
    })
}
