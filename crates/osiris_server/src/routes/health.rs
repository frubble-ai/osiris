//! `GET /health`: 200 when the database answers, 503 when it doesn't, with the build's commit.

use crate::http::{health_answer::health_answer, state::ApiState};
use axum::{Json, extract::State, http::StatusCode};
use osiris_env::Flow;
use serde_json::Value;

#[tracing::instrument(skip_all, fields(flow = %Flow::Health))]
pub async fn health(State(state): State<ApiState>) -> (StatusCode, Json<Value>) {
    health_answer(state.use_cases.check_database().await.unwrap_or(false))
}
