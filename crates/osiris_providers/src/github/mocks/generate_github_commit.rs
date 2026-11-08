//! A commit, as GitHub answers it: alone, with its stats and files, or in a pull request's list,
//! where its readers take only the fields they know.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{
    Fake,
    faker::{internet::en::SafeEmail, lorem::en::Sentence, name::en::Name},
};
use serde_json::{Value, json};

/// A commit by account `author_id`, with one parent and one changed file.
pub fn generate_github_commit(author_id: u64) -> Value {
    let sha = generate_sha();
    let additions: i64 = (0..500).fake();
    let deletions: i64 = (0..500).fake();
    json!({
        "sha": sha,
        "html_url": format!("https://github.com/octo/hello/commit/{sha}"),
        "commit": {
            "message": Sentence(3..8).fake::<String>(),
            "author": {
                "name": Name().fake::<String>(),
                "email": SafeEmail().fake::<String>(),
                "date": "2026-01-02T03:04:05Z",
            },
            "committer": {
                "name": Name().fake::<String>(),
                "email": SafeEmail().fake::<String>(),
                "date": "2026-01-02T03:04:06Z",
            },
        },
        "author": generate_github_user(author_id),
        "committer": generate_github_user(author_id),
        "parents": [{ "sha": generate_sha() }],
        "stats": { "additions": additions, "deletions": deletions, "total": additions + deletions },
        "files": [{
            "filename": "src/lib.rs",
            "status": "modified",
            "additions": additions,
            "deletions": deletions,
            "changes": additions + deletions,
            "patch": "@@ -1 +1 @@",
        }],
    })
}
