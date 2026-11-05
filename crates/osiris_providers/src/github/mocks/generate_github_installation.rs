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
    })
}
