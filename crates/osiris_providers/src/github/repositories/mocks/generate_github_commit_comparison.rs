//! Two commits compared, as GitHub answers it.

use crate::github::mocks::generate_github_commit::generate_github_commit;
use fake::Fake;
use serde_json::{Value, json};

/// A comparison `ahead_by` commits ahead, each of them listed.
pub fn generate_github_commit_comparison(ahead_by: u64) -> Value {
    let commits: Vec<Value> = (0..ahead_by)
        .map(|_| generate_github_commit((1..10_000).fake()))
        .collect();
    json!({
        "status": "ahead",
        "ahead_by": ahead_by,
        "behind_by": 0,
        "total_commits": ahead_by,
        "base_commit": { "sha": "1111111111111111111111111111111111111111" },
        "merge_base_commit": { "sha": "2222222222222222222222222222222222222222" },
        "html_url": "https://github.com/octo/hello/compare/main...feature",
        "commits": commits,
        "files": [{
            "filename": "README.md",
            "status": "added",
            "additions": 1,
            "deletions": 0,
            "changes": 1,
        }],
    })
}
