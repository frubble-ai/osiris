//! A team of an organization, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A team `id`, its slug made from its name.
pub fn generate_github_team(id: u64) -> Value {
    let name: String = Word().fake();
    json!({
        "id": id,
        "name": name,
        "slug": name.to_lowercase(),
    })
}
