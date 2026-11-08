//! A repository's collaborator, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use serde_json::{Value, json};

/// The account `id`, with push access.
pub fn generate_github_collaborator(id: u64) -> Value {
    let mut collaborator = generate_github_user(id);
    if let Some(fields) = collaborator.as_object_mut() {
        fields.insert(
            "permissions".into(),
            json!({ "admin": false, "maintain": false, "push": true, "triage": true, "pull": true }),
        );
        fields.insert("role_name".into(), json!("write"));
    }
    collaborator
}
