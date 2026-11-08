//! A milestone, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Sentence};
use serde_json::{Value, json};

/// Milestone `number`, whose id is `id`.
pub fn generate_github_milestone(id: u64, number: u64) -> Value {
    json!({
        "id": id,
        "number": number,
        "title": Sentence(1..4).fake::<String>(),
        "state": "open",
        "description": null,
        "due_on": "2026-12-31T00:00:00Z",
        "open_issues": (0..50).fake::<i64>(),
        "closed_issues": (0..50).fake::<i64>(),
        "html_url": format!("https://github.com/octo/hello/milestone/{number}"),
    })
}
