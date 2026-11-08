//! An event on one of GitHub's activity feeds, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use fake::{
    Fake,
    faker::{internet::en::Username, lorem::en::Word},
};
use serde_json::{Value, json};

/// Event `id` of type `event_type`, by a generated actor in a generated repository.
pub fn generate_github_event(id: u64, event_type: &str) -> Value {
    let repository = format!(
        "{}/{}",
        Username().fake::<String>(),
        Word().fake::<String>()
    );
    json!({
        "id": id.to_string(),
        "type": event_type,
        "actor": generate_github_user(id + 1000),
        "repo": {
            "id": id + 2000,
            "name": repository,
            "url": format!("https://api.github.com/repos/{repository}"),
        },
        "payload": { "action": "started" },
        "public": true,
        "created_at": "2026-11-01T10:00:00Z",
    })
}
