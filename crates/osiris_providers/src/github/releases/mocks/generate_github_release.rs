//! A release, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{
    Fake,
    faker::lorem::en::{Sentence, Word},
};
use serde_json::{Value, json};

/// Release `id` of the tag `tag_name`, published, written by a generated author.
pub fn generate_github_release(id: u64, tag_name: &str) -> Value {
    json!({
        "id": id,
        "tag_name": tag_name,
        "name": Word().fake::<String>(),
        "body": Sentence(3..8).fake::<String>(),
        "draft": false,
        "prerelease": false,
        "html_url": format!("https://github.com/octo/hello/releases/tag/{tag_name}"),
        "assets_url": format!("https://api.github.com/repos/octo/hello/releases/{id}/assets"),
        "upload_url": format!("https://uploads.github.com/repos/octo/hello/releases/{id}/assets{{?name,label}}"),
        "assets": [],
        "tarball_url": format!("https://api.github.com/repos/octo/hello/tarball/{tag_name}"),
        "zipball_url": format!("https://api.github.com/repos/octo/hello/zipball/{tag_name}"),
        "target_commitish": generate_sha(),
        "author": generate_github_user(id + 1000),
        "created_at": "2026-11-01T10:00:00Z",
        "published_at": "2026-11-01T10:05:00Z",
        "updated_at": "2026-11-01T10:06:00Z",
        "immutable": false,
    })
}
