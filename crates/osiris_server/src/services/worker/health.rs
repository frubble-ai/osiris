//! `GET /health`: 200 when the database answers, 503 when it doesn't or once SIGTERM has arrived
//! (`draining`, while tasks already started finish), with the build's commit.

use super::state::WorkerState;
use crate::http::health_answer::health_answer;
use axum::{Json, extract::State, http::StatusCode};
use osiris_env::Flow;
use serde_json::{Value, json};
use std::sync::atomic::Ordering;

#[tracing::instrument(skip_all, fields(flow = %Flow::Health))]
pub async fn health(State(state): State<WorkerState>) -> (StatusCode, Json<Value>) {
    if state.draining.load(Ordering::SeqCst) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "draining", "commit": osiris_env::build::commit() })),
        );
    }
    health_answer(state.use_cases.check_database().await.unwrap_or(false))
}
