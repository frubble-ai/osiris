//! An answer in the current release's shape, taken back to the older release the request was
//! written for. An answer that isn't JSON is passed on as it is.

use axum::{
    body::{Body, to_bytes},
    http::header::CONTENT_LENGTH,
    response::Response,
};
use osiris_api_models::version::{Release, Route, translate::response_down};

/// `response`, in the current release's shape, with its JSON body taken back to `release`.
pub async fn translate_response(
    releases: &[Release],
    release: &Release,
    route: &Route<'_>,
    response: Response,
) -> Response {
    let (mut parts, body) = response.into_parts();
    let Ok(bytes) = to_bytes(body, usize::MAX).await else {
        tracing::error!("an answer's body could not be read to translate it");
        return Response::from_parts(parts, Body::empty());
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
