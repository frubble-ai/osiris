//! A repository, as GitHub answers it.

use fake::{
    Fake,
    faker::{internet::en::Username, lorem::en::Word},
};
use serde_json::{Value, json};

pub fn generate_github_repository(id: u64) -> Value {
    let full_name = format!(
        "{}/{}",
        Username().fake::<String>(),
        Word().fake::<String>()
    );
    let (owner, name) = full_name.split_once('/').unwrap_or_default();
    json!({
        "id": id,
        "name": name,
        "full_name": full_name,
        "private": (0..2).fake::<u8>() == 1,
        "html_url": format!("https://github.com/{full_name}"),
        "clone_url": format!("https://github.com/{full_name}.git"),
        "ssh_url": format!("git@github.com:{full_name}.git"),
        "description": null,
        "default_branch": "main",
        "visibility": "public",
        "fork": false,
        "homepage": "https://example.com",
        "language": "Rust",
        "topics": ["osiris", "seed"],
        "license": {"key": "mit", "name": "MIT License", "spdx_id": "MIT", "url": "https://api.github.com/licenses/mit"},
        "archived": false,
        "disabled": false,
        "is_template": true,
        "has_issues": true,
        "has_projects": false,
        "has_wiki": true,
        "has_pages": false,
        "has_discussions": true,
        "stargazers_count": 12,
        "watchers_count": 12,
        "forks_count": 3,
        "open_issues_count": 4,
        "size": 260,
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-02-01T00:00:00Z",
        "pushed_at": "2024-02-02T00:00:00Z",
        "permissions": {"admin": true, "maintain": true, "push": true, "triage": true, "pull": true},
        "owner": {
            "id": id + 1000,
            "login": owner,
            "avatar_url": format!("https://avatars.githubusercontent.com/u/{}", id + 1000),
            "html_url": format!("https://github.com/{owner}"),
            "type": "User",
        },
    })
}
