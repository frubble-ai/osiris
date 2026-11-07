//! A request written for an older release, brought up to the current one: its JSON body and its
//! query through the version changes made since. A body that isn't JSON is passed on as it is, for
//! the route to refuse.

use crate::http::error::ApiError;
use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{Uri, header::CONTENT_LENGTH},
};
use osiris_api_models::version::{
    Release, Route,
    translate::{query_up, request_up},
};

/// `request`, written for `release`, with its query and JSON body brought up to the current
/// release; a body over `max_body` is refused.
pub async fn translate_request(
    releases: &[Release],
    release: &Release,
    route: &Route<'_>,
    max_body: usize,
    request: Request,
) -> Result<Request, ApiError> {
    let (mut parts, body) = request.into_parts();
    if let Some(query) = parts.uri.query() {
        let mut pairs: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();
        query_up(releases, release, route, &mut pairs);
        let query = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(pairs)
            .finish();
        let path_and_query = if query.is_empty() {
            parts.uri.path().to_owned()
        } else {
            format!("{}?{query}", parts.uri.path())
        };
        parts.uri = Uri::try_from(path_and_query)
            .map_err(|_| ApiError::Invalid("the query is not valid".into()))?;
    }
    let bytes = to_bytes(body, max_body)
        .await
        .map_err(|_| ApiError::Invalid("the body is too large".into()))?;
    let Ok(mut json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return Ok(Request::from_parts(parts, Body::from(bytes)));
    };
    request_up(releases, release, route, &mut json);
    let bytes = serde_json::to_vec(&json).unwrap_or_default();
    parts.headers.remove(CONTENT_LENGTH);
    Ok(Request::from_parts(parts, Body::from(bytes)))
}
