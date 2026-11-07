//! An answer in the current release's shape, taken back to the older release the request was
//! written for. An answer that isn't JSON is passed on as it is. One larger than
//! `[server] max_answer_bytes`, or one that can't be read, answers 500, since it can't be taken
//! back and the current release's shape is not what the client asked for.

use crate::http::error::ApiError;
use axum::{
    body::{Body, to_bytes},
    http::header::CONTENT_LENGTH,
    response::{IntoResponse, Response},
};
use error_stack::Report;
use osiris_domain::DomainError;
use osiris_versioning::{Release, Route, translate::response_down};

/// `response`, in the current release's shape, with its JSON body taken back to `release`; an
/// answer over `max_answer` bytes answers 500.
pub async fn translate_response(
    releases: &[Release],
    release: &Release,
    route: &Route<'_>,
    max_answer: usize,
    response: Response,
) -> Response {
    let (mut parts, body) = response.into_parts();
    let Ok(bytes) = to_bytes(body, max_answer).await else {
        tracing::error!(
            max_answer,
            "an answer could not be read whole to take it back to an older release"
        );
        return ApiError::Domain(Report::new(DomainError::Internal)).into_response();
    };
    let Ok(mut json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return Response::from_parts(parts, Body::from(bytes));
    };
    response_down(releases, release, route, parts.status.as_u16(), &mut json);
    parts.headers.remove(CONTENT_LENGTH);
    Response::from_parts(
        parts,
        Body::from(serde_json::to_vec(&json).unwrap_or_default()),
    )
}
