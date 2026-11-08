//! A user's public profile, as GitHub answers it at `GET /users/{username}`.

use crate::github::mocks::generate_github_user::generate_github_current_user;
use fake::Fake;
use serde_json::{Value, json};

/// User `id`'s public profile, with followers and repositories.
pub fn generate_github_public_user(id: u64) -> Value {
    let mut user = generate_github_current_user(id);
    if let Some(fields) = user.as_object_mut() {
        fields.insert("followers".into(), json!((0..10_000).fake::<i64>()));
        fields.insert("following".into(), json!((0..1000).fake::<i64>()));
        fields.insert("public_repos".into(), json!((0..500).fake::<i64>()));
    }
    user
}
