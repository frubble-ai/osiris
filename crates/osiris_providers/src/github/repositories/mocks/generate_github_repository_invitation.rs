//! An invitation to collaborate on a repository, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use serde_json::{Value, json};

/// Invitation `id`, of account `invitee_id`, to `permissions`.
pub fn generate_github_repository_invitation(id: u64, invitee_id: u64, permissions: &str) -> Value {
    json!({
        "id": id,
        "permissions": permissions,
        "html_url": "https://github.com/octo/hello/invitations",
        "invitee": generate_github_user(invitee_id),
        "inviter": generate_github_user(invitee_id + 1),
        "created_at": "2026-01-02T03:04:05Z",
        "expired": false,
    })
}
