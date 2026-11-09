//! A commit a search found, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{Fake, faker::lorem::en::Sentence};
use serde_json::{Value, json};

/// A commit in repository `repository_id`, by generated users.
pub fn generate_github_search_commit(repository_id: u64) -> Value {
    let sha = generate_sha();
    json!({
        "sha": sha,
        "url": format!("https://api.github.com/repos/octo/hello/commits/{sha}"),
        "html_url": format!("https://github.com/octo/hello/commit/{sha}"),
        "commit": {
            "message": Sentence(3..8).fake::<String>(),
            "author": { "name": "Mona", "email": "mona@example.com", "date": "2026-01-02T03:04:05Z" },
            "committer": { "name": "GitHub", "email": "noreply@github.com", "date": "2026-01-02T03:04:06Z" },
        },
        "parents": [{ "sha": generate_sha() }],
        "author": generate_github_user(repository_id + 1),
        "committer": generate_github_user(repository_id + 2),
        "repository": {
            "id": repository_id,
            "full_name": "octo/hello",
            "html_url": "https://github.com/octo/hello",
        },
        "score": 1.0,
    })
}
