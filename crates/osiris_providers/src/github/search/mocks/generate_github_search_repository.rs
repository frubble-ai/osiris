//! A repository a search found, as GitHub answers it.

use crate::github::mocks::generate_github_repository::generate_github_repository;
use fake::Fake;
use serde_json::{Value, json};

/// Repository `id`, with a star count, a language and a score.
pub fn generate_github_search_repository(id: u64) -> Value {
    let mut repository = generate_github_repository(id);
    if let Some(fields) = repository.as_object_mut() {
        fields.insert("stargazers_count".into(), json!((0..100_000).fake::<i64>()));
        fields.insert("language".into(), json!("Rust"));
        fields.insert("score".into(), json!(1.0));
    }
    repository
}
