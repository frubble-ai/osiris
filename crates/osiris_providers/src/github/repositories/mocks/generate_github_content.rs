//! A repository's contents, as GitHub answers them: a directory's entries, a file with its
//! content, and the commit a write or a delete made.

use crate::github::mocks::generate_sha::generate_sha;
use base64::{Engine, engine::general_purpose::STANDARD};
use fake::{Fake, faker::lorem::en::Sentence};
use serde_json::{Value, json};

/// An entry at `path` of type `entry_type` (`file`, `dir`, `symlink` or `submodule`).
pub fn generate_github_content_entry(path: &str, entry_type: &str) -> Value {
    let name = path.rsplit('/').next().unwrap_or(path);
    json!({
        "type": entry_type,
        "name": name,
        "path": path,
        "sha": generate_sha(),
        "size": (0..100_000).fake::<i64>(),
        "html_url": format!("https://github.com/octo/hello/blob/main/{path}"),
        "download_url": if entry_type == "file" {
            Value::String(format!("https://raw.githubusercontent.com/octo/hello/main/{path}"))
        } else {
            Value::Null
        },
    })
}

/// The file at `path` holding `bytes`, base64-encoded and wrapped every 60 characters as GitHub
/// wraps it.
pub fn generate_github_file_content(path: &str, bytes: &[u8]) -> Value {
    let encoded = STANDARD.encode(bytes);
    let wrapped: Vec<String> = encoded
        .as_bytes()
        .chunks(60)
        .map(|chunk| String::from_utf8_lossy(chunk).into_owned())
        .collect();
    let mut file = generate_github_content_entry(path, "file");
    if let Some(fields) = file.as_object_mut() {
        fields.insert("encoding".into(), Value::String("base64".into()));
        fields.insert("content".into(), Value::String(wrapped.join("\n") + "\n"));
    }
    file
}

/// A text a file may hold.
pub fn generate_text() -> String {
    Sentence(5..20).fake()
}

/// The answer to a write (`path` given, the file as it now is) or a delete (`None`).
pub fn generate_github_content_change(path: Option<&str>) -> Value {
    let sha = generate_sha();
    json!({
        "content": path.map(|path| generate_github_content_entry(path, "file")),
        "commit": {
            "sha": sha,
            "html_url": format!("https://github.com/octo/hello/commit/{sha}"),
            "message": Sentence(3..8).fake::<String>(),
        },
    })
}
