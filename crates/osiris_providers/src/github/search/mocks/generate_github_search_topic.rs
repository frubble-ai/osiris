//! A topic a search found, as GitHub answers it.

use fake::{
    Fake,
    faker::lorem::en::{Sentence, Word},
};
use serde_json::{Value, json};

/// A topic; `featured` as asked.
pub fn generate_github_search_topic(featured: bool) -> Value {
    let name: String = Word().fake();
    json!({
        "name": name,
        "display_name": name,
        "short_description": Sentence(3..6).fake::<String>(),
        "description": null,
        "featured": featured,
        "curated": false,
        "score": 1.0,
    })
}
