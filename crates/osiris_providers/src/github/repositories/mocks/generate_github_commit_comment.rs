//! A comment on a commit, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use fake::{Fake, faker::lorem::en::Sentence};
use serde_json::{Value, json};

/// Comment `id` by account `author_id`, on line `position` of `path`.
pub fn generate_github_commit_comment(id: u64, author_id: u64, path: &str, position: u32) -> Value {
    json!({
        "id": id,
        "body": Sentence(3..10).fake::<String>(),
        "html_url": format!("https://github.com/octo/hello/commit/abc#commitcomment-{id}"),
        "path": path,
        "position": position,
        "user": generate_github_user(author_id),
        "created_at": "2026-01-02T03:04:05Z",
        "updated_at": "2026-01-02T03:04:05Z",
    })
}
