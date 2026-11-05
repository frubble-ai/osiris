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
    json!({
        "id": id,
        "full_name": full_name,
        "private": (0..2).fake::<u8>() == 1,
        "html_url": format!("https://github.com/{full_name}"),
        "description": null,
    })
}
