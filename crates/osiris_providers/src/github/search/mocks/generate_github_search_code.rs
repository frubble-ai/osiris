//! A file a code search found, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A Rust file in repository `repository_id`.
pub fn generate_github_search_code(repository_id: u64) -> Value {
    let name = format!("{}.rs", Word().fake::<String>());
    let sha = generate_sha();
    json!({
        "name": name,
        "path": format!("src/{name}"),
        "sha": sha,
        "url": format!("https://api.github.com/repos/octo/hello/contents/src/{name}"),
        "git_url": format!("https://api.github.com/repos/octo/hello/git/blobs/{sha}"),
        "html_url": format!("https://github.com/octo/hello/blob/main/src/{name}"),
        "repository": {
            "id": repository_id,
            "full_name": "octo/hello",
            "html_url": "https://github.com/octo/hello",
            "owner": generate_github_user(repository_id + 1),
        },
        "score": 1.0,
    })
}
