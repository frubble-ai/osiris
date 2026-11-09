//! An installation of the GitHub App, as GitHub answers it.

use fake::{Fake, faker::internet::en::Username};
use serde_json::{Value, json};

/// An installation on a user account (`organization` false) or an organization.
pub fn generate_github_installation(id: u64, organization: bool) -> Value {
    json!({
        "id": id,
        "account": {
            "login": Username().fake::<String>(),
            "type": if organization { "Organization" } else { "User" },
        },
        "repository_selection": if organization { "selected" } else { "all" },
        "app_id": 1,
        "app_slug": "frubble-ai-dev",
        "target_type": if organization { "Organization" } else { "User" },
        "permissions": { "contents": "read", "issues": "write", "metadata": "read" },
        "events": ["push", "pull_request"],
        "created_at": "2026-01-01T00:00:00.000Z",
        "updated_at": "2026-01-02T00:00:00.000Z",
        "suspended_at": null,
    })
}
