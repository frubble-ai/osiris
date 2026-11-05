//! The API's router: `/v1` behind the body limit, the version header and idempotency keys; the
//! OAuth callback and `/health` in the open.

use super::v1::v1;
use crate::{
    http::{idempotency::idempotency, state::ApiState, version::version},
    routes,
};
use axum::{Router, extract::DefaultBodyLimit, middleware, routing::get};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

pub fn router(state: ApiState) -> Router {
    let timeout = state.core.settings.server.request_timeout();
    let max_body = state.core.settings.server.max_body();
    let (v1, _) = v1()
        .layer(middleware::from_fn_with_state(state.clone(), idempotency))
        .layer(middleware::from_fn_with_state(state.clone(), version))
        .layer(DefaultBodyLimit::max(max_body))
        .split_for_parts();
    Router::new()
        .route("/health", get(routes::health::health))
        .route("/oauth/callback", get(routes::oauth_callback::oauth_callback))
        .merge(v1)
        .layer(TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!("request", method = %request.method(), path = %request.uri().path(), request_id = %osiris_env::request_id::of(request))
        }))
        .layer(osiris_env::request_id::propagate_layer())
        .layer(osiris_env::request_id::set_layer())
        .layer(TimeoutLayer::with_status_code(axum::http::StatusCode::REQUEST_TIMEOUT, timeout))
        .with_state(state)
}
