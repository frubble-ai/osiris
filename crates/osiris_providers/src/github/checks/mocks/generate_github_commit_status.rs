//! A commit status, as GitHub answers it.

use fake::{
    Fake,
    faker::{internet::en::Username, lorem::en::Sentence},
};
use serde_json::{Value, json};

/// A status `id` in `state`, set by a generated service.
pub fn generate_github_commit_status(id: u64, state: &str) -> Value {
    let context = format!("ci/{}", Username().fake::<String>());
    json!({
        "id": id,
        "state": state,
        "context": context,
        "description": Sentence(3..6).fake::<String>(),
        "target_url": format!("https://ci.example.com/builds/{id}"),
        "created_at": "2024-01-02T03:04:05Z",
        "updated_at": "2024-01-02T03:05:06Z",
    })
}
