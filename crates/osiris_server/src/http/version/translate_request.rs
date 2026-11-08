//! A request written for an older release, brought up to the current one: its JSON body and its
//! query through the version changes made since, which see an empty query and an empty object when
//! the request has none, so a change can add a default. A body that isn't JSON is passed on as it
//! is, for the route to refuse.

use crate::http::error::ApiError;
use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{
        HeaderValue, Uri,
        header::{CONTENT_LENGTH, CONTENT_TYPE},
    },
};
use osiris_versioning::{
    Release, Route,
    translate::{query_up, request_up},
};
use serde_json::{Map, Value};

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
    // A change may add a default an older release had, so it sees the query and the body even
    // when the request has none; an empty one it leaves empty stays empty.
    let had_query = parts.uri.query().is_some();
    let mut pairs: Vec<(String, String)> = parts
        .uri
        .query()
        .map(|query| {
            url::form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_default();
    query_up(releases, release, route, &mut pairs);
    if had_query || !pairs.is_empty() {
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
    let empty = bytes.is_empty();
    let mut json = if empty {
        Value::Object(Map::new())
    } else {
        match serde_json::from_slice::<Value>(&bytes) {
            Ok(json) => json,
            Err(_) => return Ok(Request::from_parts(parts, Body::from(bytes))),
        }
    };
    request_up(releases, release, route, &mut json);
    if empty && json.as_object().is_some_and(Map::is_empty) {
        return Ok(Request::from_parts(parts, Body::empty()));
    }
    let bytes = serde_json::to_vec(&json).unwrap_or_default();
    parts.headers.remove(CONTENT_LENGTH);
    parts
        .headers
        .entry(CONTENT_TYPE)
        .or_insert(HeaderValue::from_static("application/json"));
    Ok(Request::from_parts(parts, Body::from(bytes)))
}

#[cfg(test)]
mod tests;
