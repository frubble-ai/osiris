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
        "commit_id": "6b6f6f3373bddf70d592019006ad64aec76fa8ae",
        "line": 3,
        "author_association": "OWNER",
        "reactions": { "total_count": 2, "+1": 1, "-1": 0, "laugh": 0, "confused": 0, "heart": 1, "hooray": 0, "rocket": 0, "eyes": 0 },
    })
}
