//! The worker's router: `/health` and the task endpoint.

use super::{health::health, state::WorkerState, tasks::run_task};
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

pub fn router(state: WorkerState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", post(run_task))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
