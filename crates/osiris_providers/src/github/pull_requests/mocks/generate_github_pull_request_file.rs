//! A file a pull request changes, as GitHub answers it.

use crate::github::mocks::generate_sha::generate_sha;
use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A modified file with `additions` lines added and `deletions` removed.
pub fn generate_github_pull_request_file(additions: i64, deletions: i64) -> Value {
    let sha = generate_sha();
    json!({
        "sha": sha,
        "filename": format!("src/{}.rs", Word().fake::<String>()),
        "status": "modified",
        "additions": additions,
        "deletions": deletions,
        "changes": additions + deletions,
        "patch": "@@ -1 +1 @@\n-old\n+new",
        "blob_url": format!("https://github.com/octo/hello/blob/{sha}/src/main.rs"),
    })
}
