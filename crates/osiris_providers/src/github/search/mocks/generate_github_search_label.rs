//! A label a search found, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// Label `id`, without a description.
pub fn generate_github_search_label(id: u64) -> Value {
    json!({
        "id": id,
        "name": Word().fake::<String>(),
        "color": format!("{:06x}", (0..0x00ff_ffff).fake::<u32>()),
        "description": null,
        "default": true,
        "score": 1.0,
    })
}
