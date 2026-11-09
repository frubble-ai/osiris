//! A workflow, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// An active workflow `id`, kept in a generated file.
pub fn generate_github_workflow(id: u64) -> Value {
    let file = format!("{}.yml", Word().fake::<String>());
    json!({
        "id": id,
        "name": Word().fake::<String>(),
        "path": format!(".github/workflows/{file}"),
        "state": "active",
        "html_url": format!("https://github.com/octo/hello/blob/main/.github/workflows/{file}"),
        "badge_url": format!("https://github.com/octo/hello/workflows/{id}/badge.svg"),
        "created_at": "2024-01-01T00:00:00.000-03:00",
        "updated_at": "2024-01-02T00:00:00.000-03:00",
    })
}
