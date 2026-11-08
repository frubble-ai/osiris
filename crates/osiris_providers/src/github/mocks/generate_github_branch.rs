//! A branch, as GitHub answers it.

use super::generate_sha::generate_sha;
use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A branch whose head is a generated commit; `protected` as asked.
pub fn generate_github_branch(protected: bool) -> Value {
    let sha = generate_sha();
    json!({
        "name": Word().fake::<String>(),
        "commit": { "sha": sha, "url": format!("https://api.github.com/commits/{sha}") },
        "protected": protected,
    })
}
