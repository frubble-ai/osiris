//! A comment on a line of a pull request's diff, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{
    Fake,
    faker::lorem::en::{Sentence, Word},
};
use serde_json::{Value, json};

/// A comment `id` on a line of the new version of a file; a reply to `in_reply_to` if given.
pub fn generate_github_review_comment(id: u64, in_reply_to: Option<u64>) -> Value {
    let sha = generate_sha();
    json!({
        "id": id,
        "path": format!("src/{}.rs", Word().fake::<String>()),
        "body": Sentence(3..8).fake::<String>(),
        "user": generate_github_user(id + 1000),
        "commit_id": sha,
        "original_commit_id": sha,
        "diff_hunk": "@@ -1 +1 @@",
        "html_url": format!("https://github.com/octo/hello/pull/1#discussion_r{id}"),
        "line": (1..500).fake::<i64>(),
        "original_line": (1..500).fake::<i64>(),
        "start_line": null,
        "side": "RIGHT",
        "start_side": null,
        "in_reply_to_id": in_reply_to,
        "pull_request_review_id": id + 5000,
        "created_at": "2024-02-01T10:00:00Z",
        "updated_at": "2024-02-02T10:00:00Z",
        "author_association": "OWNER",
        "subject_type": "line",
        "original_start_line": null,
        "reactions": { "total_count": 2, "+1": 1, "-1": 0, "laugh": 0, "confused": 0, "heart": 1, "hooray": 0, "rocket": 0, "eyes": 0 },
    })
}
