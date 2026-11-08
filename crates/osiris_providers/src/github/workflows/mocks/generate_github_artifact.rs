//! An artifact a workflow run kept, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// An artifact `id` that has not expired.
pub fn generate_github_artifact(id: u64) -> Value {
    json!({
        "id": id,
        "name": Word().fake::<String>(),
        "size_in_bytes": (1..1_000_000).fake::<i64>(),
        "archive_download_url": format!(
            "https://api.github.com/repos/octo/hello/actions/artifacts/{id}/zip"
        ),
        "expired": false,
        "created_at": "2024-01-02T03:04:05Z",
        "expires_at": "2024-04-01T03:04:05Z",
    })
}
