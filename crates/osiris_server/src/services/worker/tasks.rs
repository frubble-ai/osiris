//! `POST /tasks`: a scheduled task, sent by the task scheduler when it is due.
//!
//! Every request is authenticated before anything runs, whatever the platform does: when deployed,
//! the Google OIDC token Cloud Tasks sends (issuer, audience, service account); locally, the
//! worker token. A request that fails answers 401.
//!
//! A 204 tells the scheduler the task is done: it ran, it was out of date, or it can never be run
//! (a body that is not a task). A 500 makes the scheduler run the same task again later.

use super::state::WorkerState;
use crate::use_cases::cases::run_task::RunTaskArgs;
use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
};
use osiris_env::Flow;
use osiris_tasks::Task;

#[tracing::instrument(skip_all, fields(flow = %Flow::TasksRun))]
pub async fn run_task(
    State(state): State<WorkerState>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let authorization = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    if let Err(error) = state.authenticator.authenticate(authorization).await {
        tracing::warn!(%error, "refusing a task from a caller that could not be authenticated");
        return StatusCode::UNAUTHORIZED;
    }
    let task: Task = match serde_json::from_slice(&body) {
        Ok(task) => task,
        Err(error) => {
            // Sending it again would never help, so it is acknowledged instead of retried.
            tracing::error!(%error, "dropping a request that is not a task");
            return StatusCode::NO_CONTENT;
        }
    };
    match state.use_cases.run_task(RunTaskArgs { task: &task }).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
