//! An issue, or a pull request answered as an issue, as GitHub answers it.

use super::{
    generate_github_label::generate_github_label,
    generate_github_milestone::generate_github_milestone,
};
use crate::github::mocks::generate_github_user::generate_github_user;
use fake::{
    Fake,
    faker::lorem::en::{Paragraph, Sentence},
};
use serde_json::{Value, json};

/// Issue `number`, whose id is `id`, with an author, an assignee, a label and a milestone; a pull
/// request when `pull_request` is true.
pub fn generate_github_issue(id: u64, number: u64, pull_request: bool) -> Value {
    let mut issue = json!({
        "id": id,
        "number": number,
        "title": Sentence(2..6).fake::<String>(),
        "state": "open",
        "html_url": format!("https://github.com/octo/hello/issues/{number}"),
        "body": Paragraph(1..3).fake::<String>(),
        "comments": (0..20).fake::<i64>(),
        "user": generate_github_user(id + 1),
        "assignees": [generate_github_user(id + 2)],
        "labels": [generate_github_label(id + 3)],
        "milestone": generate_github_milestone(id + 4, 1),
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-02T00:00:00Z",
        "closed_at": null,
        "locked": false,
        "active_lock_reason": null,
        "state_reason": null,
    });
    if pull_request && let Some(fields) = issue.as_object_mut() {
        fields.insert(
            "pull_request".into(),
            json!({ "url": format!("https://api.github.com/repos/octo/hello/pulls/{number}") }),
        );
    }
    issue
}
