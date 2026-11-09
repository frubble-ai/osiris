//! A comment on an issue, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use fake::{Fake, faker::lorem::en::Paragraph};
use serde_json::{Value, json};

/// Comment `id`, by a generated author.
pub fn generate_github_issue_comment(id: u64) -> Value {
    json!({
        "id": id,
        "html_url": format!("https://github.com/octo/hello/issues/1#issuecomment-{id}"),
        "body": Paragraph(1..3).fake::<String>(),
        "user": generate_github_user(id + 1),
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-02T00:00:00Z",
        "reactions": { "total_count": 1, "+1": 0, "-1": 1, "laugh": 0, "confused": 0, "heart": 0, "hooray": 0, "rocket": 0, "eyes": 0 },
    })
}
