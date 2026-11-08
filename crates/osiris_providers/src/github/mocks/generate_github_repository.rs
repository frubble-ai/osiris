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
        "owner": {
            "id": id + 1000,
            "login": owner,
            "avatar_url": format!("https://avatars.githubusercontent.com/u/{}", id + 1000),
            "html_url": format!("https://github.com/{owner}"),
            "type": "User",
        },
    })
}
