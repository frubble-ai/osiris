//! A fork's branch synced with its upstream, as GitHub answers it.

use fake::{Fake, faker::internet::en::Username};
use serde_json::{Value, json};

/// `branch` fast-forwarded from an upstream of a generated owner.
pub fn generate_github_fork_sync(branch: &str) -> Value {
    let upstream = format!("{}:{branch}", Username().fake::<String>());
    json!({
        "message": format!("Successfully fetched and fast-forwarded from upstream {upstream}."),
        "merge_type": "fast-forward",
        "base_branch": upstream,
    })
}
