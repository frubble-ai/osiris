//! A review of a pull request, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{Fake, faker::lorem::en::Sentence};
use serde_json::{Value, json};

/// A review `id` in `state` (`APPROVED`, `PENDING`, `DISMISSED` and so on).
pub fn generate_github_review(id: u64, state: &str) -> Value {
    json!({
        "id": id,
        "user": generate_github_user(id + 1000),
        "body": Sentence(3..8).fake::<String>(),
        "state": state,
        "html_url": format!("https://github.com/octo/hello/pull/1#pullrequestreview-{id}"),
        "commit_id": generate_sha(),
        "submitted_at": "2024-01-01T00:00:00Z",
    })
}
