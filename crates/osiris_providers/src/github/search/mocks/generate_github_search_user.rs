//! A user a search found, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use serde_json::{Value, json};

/// Account `id`, with a score.
pub fn generate_github_search_user(id: u64) -> Value {
    let mut user = generate_github_user(id);
    if let Some(fields) = user.as_object_mut() {
        fields.insert("score".into(), json!(1.0));
    }
    user
}
