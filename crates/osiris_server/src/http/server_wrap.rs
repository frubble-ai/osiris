//! The one wrapper every `/v1` route goes through: the key, the
//! flow's span with the request's id, the outcome logged with its time, and the error mapped to the
//! wire. A route only parses its request and names the use case to run. Secrets are typed
//! `Secret`, so nothing logged here can carry one.

use super::{
    auth::{Auth, check},
    error::ApiError,
    state::ApiState,
};
use crate::use_cases::{UseCaseResult, UseCases};
use axum::{
    Json,
    http::{HeaderMap, StatusCode},
};
use osiris_env::Flow;
use serde::Serialize;
use std::time::Instant;
use tracing::Instrument;

pub async fn server_wrap<T, F, Fut>(
    flow: Flow,
    state: &ApiState,
    headers: &HeaderMap,
    auth: Auth,
    status: StatusCode,
    run: F,
) -> Result<(StatusCode, Json<T>), ApiError>
where
    T: Serialize,
    F: FnOnce(UseCases) -> Fut,
    Fut: Future<Output = UseCaseResult<T>>,
{
    let request_id = headers
        .get(osiris_env::request_id::HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("none")
        .to_owned();
    let span = tracing::info_span!("flow", %flow, %request_id);
    async move {
        let started = Instant::now();
        check(state, headers, auth)?;
        let outcome = run(state.use_cases.clone()).await;
        let elapsed_ms = started.elapsed().as_millis();
        match outcome {
            Ok(answer) => {
                tracing::info!(elapsed_ms, status = status.as_u16(), "done");
                Ok((status, Json(answer)))
            }
            Err(report) => {
                tracing::info!(elapsed_ms, error = %report.current_context(), "failed");
                Err(ApiError::Domain(report))
            }
        }
    }
    .instrument(span)
    .await
}
