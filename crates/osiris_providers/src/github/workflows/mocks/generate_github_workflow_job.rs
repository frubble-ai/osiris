//! A job of a workflow run, as GitHub answers it.

use fake::{Fake, faker::lorem::en::Word};
use serde_json::{Value, json};

/// A finished job `id` of run `run_id`, with one step.
pub fn generate_github_workflow_job(id: u64, run_id: u64) -> Value {
    json!({
        "id": id,
        "run_id": run_id,
        "name": Word().fake::<String>(),
        "status": "completed",
        "conclusion": "failure",
        "html_url": format!("https://github.com/octo/hello/actions/runs/{run_id}/job/{id}"),
        "started_at": "2024-01-02T03:04:05Z",
        "completed_at": "2024-01-02T03:09:05Z",
        "steps": [{
            "number": 1,
            "name": Word().fake::<String>(),
            "status": "completed",
            "conclusion": "failure",
            "started_at": "2024-01-02T03:04:05Z",
            "completed_at": "2024-01-02T03:09:05Z",
        }],
    })
}
