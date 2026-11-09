//! A pull request, as GitHub answers it.

use crate::github::mocks::{
    generate_github_user::generate_github_user, generate_sha::generate_sha,
};
use fake::{
    Fake,
    faker::{
        internet::en::Username,
        lorem::en::{Sentence, Word},
    },
};
use serde_json::{Value, json};

/// An open pull request `id`, numbered `number`, from a generated branch into `main`.
pub fn generate_github_pull_request(id: u64, number: u64) -> Value {
    let full_name = format!(
        "{}/{}",
        Username().fake::<String>(),
        Word().fake::<String>()
    );
    let head = Word().fake::<String>();
    json!({
        "id": id,
        "number": number,
        "state": "open",
        "title": Sentence(3..6).fake::<String>(),
        "body": Sentence(5..10).fake::<String>(),
        "html_url": format!("https://github.com/{full_name}/pull/{number}"),
        "draft": false,
        "user": generate_github_user(id + 1000),
        "head": {
            "ref": head,
            "sha": generate_sha(),
            "label": format!("octo:{head}"),
            "repo": { "full_name": full_name },
        },
        "base": {
            "ref": "main",
            "sha": generate_sha(),
            "label": "octo:main",
            "repo": { "full_name": full_name },
        },
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-02T00:00:00Z",
        "closed_at": null,
        "merged_at": null,
        "locked": true,
        "active_lock_reason": "resolved",
        "merged_by": null,
        "rebaseable": true,
        "author_association": "COLLABORATOR",
    })
}
