//! An account's permission in a repository, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use serde_json::{Value, json};

/// Account `id`'s `permission`.
pub fn generate_github_repository_permission(id: u64, permission: &str) -> Value {
    json!({
        "permission": permission,
        "role_name": permission,
        "user": generate_github_user(id),
    })
}
