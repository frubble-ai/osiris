//! A workflow run, as GitHub answers it.

use crate::github::mocks::generate_sha::generate_sha;
use fake::{
    Fake,
    faker::lorem::en::{Sentence, Word},
};
use serde_json::{Value, json};

/// A finished run `id` of workflow `workflow_id`, on a generated commit.
pub fn generate_github_workflow_run(id: u64, workflow_id: u64) -> Value {
    json!({
        "id": id,
        "name": Word().fake::<String>(),
        "display_title": Sentence(2..5).fake::<String>(),
        "workflow_id": workflow_id,
        "event": "push",
        "status": "completed",
        "conclusion": "success",
        "head_branch": "main",
        "head_sha": generate_sha(),
        "html_url": format!("https://github.com/octo/hello/actions/runs/{id}"),
        "run_number": (1..1000).fake::<i64>(),
        "created_at": "2024-01-02T03:04:05Z",
        "updated_at": "2024-01-02T03:09:05Z",
    })
}
