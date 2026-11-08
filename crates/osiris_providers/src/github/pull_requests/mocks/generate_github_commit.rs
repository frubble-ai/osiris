//! A commit, as GitHub answers it in a pull request's list of commits.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{
    Fake,
    faker::{internet::en::SafeEmail, lorem::en::Sentence, name::en::Name},
};
use serde_json::{Value, json};

/// A commit by account `author_id`, with one parent.
pub fn generate_github_commit(author_id: u64) -> Value {
    let sha = generate_sha();
    json!({
        "sha": sha,
        "html_url": format!("https://github.com/octo/hello/commit/{sha}"),
        "commit": {
            "message": Sentence(3..8).fake::<String>(),
            "author": {
                "name": Name().fake::<String>(),
                "email": SafeEmail().fake::<String>(),
                "date": "2024-01-01T00:00:00Z",
            },
            "committer": {
                "name": Name().fake::<String>(),
                "date": "2024-01-01T00:00:00Z",
            },
        },
        "author": generate_github_user(author_id),
        "committer": generate_github_user(author_id),
        "parents": [{ "sha": generate_sha() }],
    })
}
