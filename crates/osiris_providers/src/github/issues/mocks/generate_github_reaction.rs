//! A reaction, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use serde_json::{Value, json};

/// Reaction `id` of kind `content`, by a generated user.
pub fn generate_github_reaction(id: u64, content: &str) -> Value {
    json!({
        "id": id,
        "node_id": format!("REA_{id}"),
        "content": content,
        "user": generate_github_user(id + 1),
        "created_at": "2026-01-01T00:00:00Z",
    })
}
