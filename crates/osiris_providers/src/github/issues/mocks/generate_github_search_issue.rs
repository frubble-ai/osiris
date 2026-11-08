//! An issue or a pull request a search found, as GitHub answers it.

use super::generate_github_issue::generate_github_issue;
use fake::Fake;
use serde_json::{Value, json};

/// Issue `number`, whose id is `id`, as a search finds it; a pull request when `pull_request` is
/// true.
pub fn generate_github_search_issue(id: u64, number: u64, pull_request: bool) -> Value {
    let mut issue = generate_github_issue(id, number, pull_request);
    if let Some(fields) = issue.as_object_mut() {
        fields.insert(
            "repository_url".into(),
            json!("https://api.github.com/repos/octo/hello"),
        );
        fields.insert("score".into(), json!((1..100).fake::<u32>()));
    }
    issue
}
