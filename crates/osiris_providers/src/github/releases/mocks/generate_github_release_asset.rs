//! A file attached to a release, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// Asset `id`, uploaded by a generated user.
pub fn generate_github_release_asset(id: u64) -> Value {
    let name = format!("{}.tar.gz", Word().fake::<String>());
    json!({
        "id": id,
        "name": name,
        "label": null,
        "state": "uploaded",
        "content_type": "application/gzip",
        "size": (1..10_000_000).fake::<i64>(),
        "download_count": (0..1000).fake::<i64>(),
        "browser_download_url": format!("https://github.com/octo/hello/releases/download/v1/{name}"),
        "uploader": generate_github_user(id + 1000),
        "created_at": "2026-11-01T10:00:00Z",
        "updated_at": "2026-11-01T10:01:00Z",
    })
}
