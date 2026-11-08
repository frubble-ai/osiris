//! A page of a search, as GitHub answers it.

use fake::Fake;
use serde_json::{Value, json};

/// A page holding `items`, out of a generated total at least as large; `incomplete` as asked.
pub fn generate_github_search_page(items: &[Value], incomplete: bool) -> Value {
    let total = items.len() + (0..500).fake::<usize>();
    json!({
        "total_count": total,
        "incomplete_results": incomplete,
        "items": items,
    })
}
