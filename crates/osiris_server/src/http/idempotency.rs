//! `Idempotency-Key` on a POST or DELETE: the request runs once; a retry with the same key gets the
//! first answer again. Errors that may pass (5xx) free the key, so a retry runs again.
//!
//! Only a request carrying a valid key takes part: one without is passed to its route, which
//! refuses it, so nothing unauthenticated is ever stored or replayed. Keys are scoped by which key
//! sent them, the method and the path.

use super::{auth::principal, error::ApiError, state::ApiState};
use crate::use_cases::{
    UseCases,
    cases::{
        abandon_idempotent_request::AbandonIdempotentRequestArgs,
        begin_idempotent_request::BeginIdempotentRequestArgs,
        finish_idempotent_request::FinishIdempotentRequestArgs,
    },
};
use axum::{
    body::{Body, to_bytes},
    extract::{Request, State},
    http::{HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use osiris_domain::Begun;
use sha2::{Digest, Sha256};

const HEADER: &str = "idempotency-key";

pub async fn idempotency(
    State(state): State<ApiState>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    if !matches!(*request.method(), Method::POST | Method::DELETE) {
        return Ok(next.run(request).await);
    }
    let Some(key) = request
        .headers()
        .get(HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
    else {
        return Ok(next.run(request).await);
    };
    let Some(caller) = principal(&state, request.headers()) else {
        return Ok(next.run(request).await);
    };
    if key.is_empty() || key.len() > 255 {
        return Err(ApiError::Invalid(
            "Idempotency-Key must be 1 to 255 characters".into(),
        ));
    }
    let (parts, body) = request.into_parts();
    let bytes = to_bytes(body, state.config.settings.server.max_body())
        .await
        .map_err(|_| ApiError::Invalid("the body is too large".into()))?;
    let mut hash = Sha256::new();
    hash.update(parts.method.as_str());
    hash.update(parts.uri.path());
    hash.update(&bytes);
    let request_hash = hex::encode(hash.finalize());
    let scoped = format!("{caller:?} {} {} {key}", parts.method, parts.uri.path());

    let use_cases = &state.use_cases;
    match use_cases
        .begin_idempotent_request(BeginIdempotentRequestArgs {
            key: &scoped,
            request_hash: &request_hash,
        })
        .await?
    {
        Begun::New => {}
        Begun::Replay { status, body } => {
            let status = StatusCode::from_u16(status).unwrap_or(StatusCode::OK);
            let mut response = (status, axum::Json(body)).into_response();
            response
                .headers_mut()
                .insert("idempotent-replayed", HeaderValue::from_static("true"));
            return Ok(response);
        }
        Begun::Mismatch => return Err(ApiError::IdempotencyMismatch),
        Begun::InFlight => return Err(ApiError::InFlight),
    }

    let response = next
        .run(Request::from_parts(parts, Body::from(bytes)))
        .await;
    let status = response.status();
    if status.is_server_error() {
        abandon(use_cases, &scoped).await;
        return Ok(response);
    }
    let (parts, body) = response.into_parts();
    // The route's own answer, whole: the request's limit is not the response's.
    let Ok(bytes) = to_bytes(body, usize::MAX).await else {
        abandon(use_cases, &scoped).await;
        tracing::error!("reading a keyed request's answer failed");
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    };
    // Only a JSON answer can be replayed as it was; any other frees the key for a retry.
    let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        abandon(use_cases, &scoped).await;
        return Ok(Response::from_parts(parts, Body::from(bytes)));
    };
    if let Err(error) = use_cases
        .finish_idempotent_request(FinishIdempotentRequestArgs {
            key: &scoped,
            status: status.as_u16(),
            body: &json,
        })
        .await
    {
        tracing::error!(?error, "keeping the idempotent answer failed");
    }
    Ok(Response::from_parts(parts, Body::from(bytes)))
}

/// Free the key; a failure is logged, and the key stays in flight until its retention ends.
async fn abandon(use_cases: &UseCases, scoped: &str) {
    if let Err(error) = use_cases
        .abandon_idempotent_request(AbandonIdempotentRequestArgs { key: scoped })
        .await
    {
        tracing::error!(
            ?error,
            "freeing an idempotency key failed; retries answer in flight until it is purged"
        );
    }
}
