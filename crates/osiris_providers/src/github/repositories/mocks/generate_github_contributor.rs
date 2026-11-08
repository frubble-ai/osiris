//! A repository's contributor, as GitHub answers it.

use crate::github::mocks::generate_github_user::generate_github_user;
use fake::Fake;
use serde_json::{Value, json};

/// The account `id`, with some contributions.
pub fn generate_github_contributor(id: u64) -> Value {
    let mut contributor = generate_github_user(id);
    if let Some(fields) = contributor.as_object_mut() {
        fields.insert("contributions".into(), json!((1..5000).fake::<i64>()));
    }
    contributor
}
