//! Entries on an issue's timeline, as GitHub answers them.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{
    Fake,
    faker::{internet::en::SafeEmail, lorem::en::Paragraph, name::en::Name},
};
use serde_json::{Value, json};

/// A comment on the timeline: it has an id, an actor and a time.
pub fn generate_github_timeline_comment(id: u64) -> Value {
    json!({
        "id": id,
        "event": "commented",
        "actor": generate_github_user(id + 1),
        "body": Paragraph(1..2).fake::<String>(),
        "created_at": "2026-01-01T00:00:00Z",
    })
}

/// A commit on the timeline: it has no id, no actor and no time of its own.
pub fn generate_github_timeline_commit() -> Value {
    json!({
        "event": "committed",
        "sha": generate_sha(),
        "message": Paragraph(1..2).fake::<String>(),
        "author": {
            "name": Name().fake::<String>(),
            "email": SafeEmail().fake::<String>(),
            "date": "2026-01-01T00:00:00Z",
        },
    })
}
