//! The worker's router: `/health` and the task endpoint.

use super::{health::health, state::WorkerState, tasks::run_task};
use axum::{
    Router,
    routing::{get, post},
};
use tower::limit::ConcurrencyLimitLayer;
use tower_http::trace::TraceLayer;

pub fn router(state: WorkerState) -> Router {
    // At most `[tasks] max_concurrent` run at once. A task beyond the pool waits for a
    // connection up to `[database] connect_timeout_seconds`, then fails.
    let max_concurrent = state.config.settings.tasks.max_concurrent;
    Router::new()
        .route("/health", get(health))
        .route(
            "/tasks",
            post(run_task).layer(ConcurrencyLimitLayer::new(max_concurrent)),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
