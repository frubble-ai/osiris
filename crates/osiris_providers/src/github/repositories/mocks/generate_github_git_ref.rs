//! A git reference, as GitHub answers it.

use crate::github::mocks::generate_sha::generate_sha;
use serde_json::{Value, json};

/// The reference `ref_name` (with its `refs/` prefix), pointing at a generated commit.
pub fn generate_github_git_ref(ref_name: &str) -> Value {
    let sha = generate_sha();
    json!({
        "ref": ref_name,
        "node_id": "REF_kwDO",
        "url": format!("https://api.github.com/repos/octo/hello/git/{ref_name}"),
        "object": {
            "sha": sha,
            "type": "commit",
            "url": format!("https://api.github.com/repos/octo/hello/git/commits/{sha}"),
        },
    })
}
