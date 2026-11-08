//! A label, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A label `id` with a generated name and color.
pub fn generate_github_label(id: u64) -> Value {
    json!({
        "id": id,
        "node_id": format!("LA_{id}"),
        "name": Word().fake::<String>(),
        "color": format!("{:06x}", (0..0x00ff_ffff_u32).fake::<u32>()),
        "description": null,
        "default": false,
    })
}
