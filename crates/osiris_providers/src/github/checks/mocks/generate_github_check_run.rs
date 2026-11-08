//! A check run, as GitHub answers it.

use crate::github::mocks::generate_sha::generate_sha;
use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A finished check run `id` with `conclusion`, on a generated commit.
pub fn generate_github_check_run(id: u64, conclusion: &str) -> Value {
    json!({
        "id": id,
        "name": Word().fake::<String>(),
        "status": "completed",
        "conclusion": conclusion,
        "head_sha": generate_sha(),
        "html_url": format!("https://github.com/octo/hello/runs/{id}"),
        "details_url": format!("https://ci.example.com/checks/{id}"),
        "started_at": "2024-01-02T03:04:05Z",
        "completed_at": "2024-01-02T03:09:05Z",
    })
}
