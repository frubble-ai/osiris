//! A tag, as GitHub answers it in a repository's list of tags.

use crate::github::mocks::generate_sha::generate_sha;
use fake::Fake;
use serde_json::{Value, json};

/// A tag `v{major}.{minor}.{patch}`, on a generated commit.
pub fn generate_github_tag() -> Value {
    let name = format!(
        "v{}.{}.{}",
        (0..10).fake::<u8>(),
        (0..10).fake::<u8>(),
        (0..10).fake::<u8>()
    );
    let sha = generate_sha();
    json!({
        "name": name,
        "commit": { "sha": sha, "url": format!("https://api.github.com/repos/octo/hello/commits/{sha}") },
        "zipball_url": format!("https://api.github.com/repos/octo/hello/zipball/{name}"),
        "tarball_url": format!("https://api.github.com/repos/octo/hello/tarball/{name}"),
    })
}
