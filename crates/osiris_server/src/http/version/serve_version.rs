//! The version middleware: resolves the release a request is served at, translates the request
//! and its answer when that release is not the current one, and names the release and whether it
//! is a preview on the answer.

use super::{
    lifecycle_headers::lifecycle_headers, resolve_release::resolve_release,
    translate_request::translate_request, translate_response::translate_response,
};
use crate::http::{error::ApiError, state::ApiState};
use axum::{
    extract::{MatchedPath, Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use osiris_api_models::version::{HEADER, Route, STABILITY_HEADER};

/// Serves `request` at the release it names, translating it and its answer when that release
/// isn't the current one.
pub async fn serve_version(
    State(state): State<ApiState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let release = resolve_release(&state, request.headers())?;
    let method = request.method().as_str().to_owned();
    let path = request.extensions().get::<MatchedPath>().map_or_else(
        || request.uri().path().to_owned(),
        |matched| matched.as_str().to_owned(),
    );
    let route = Route {
        method: &method,
        path: &path,
    };
    let current = state
        .releases
        .last()
        .is_some_and(|last| last.name == release.name);
    if !current {
        let max_body = state.config.settings.server.max_body();
        request = translate_request(state.releases, release, &route, max_body, request).await?;
    }
    let mut response = next.run(request).await;
    if !current {
        response = translate_response(state.releases, release, &route, response).await;
    }
    let headers = response.headers_mut();
    lifecycle_headers(
        state.releases,
        release,
        state.config.settings.api.support(),
        headers,
    );
    headers.insert(HEADER, HeaderValue::from_static(release.name));
    headers.insert(
        STABILITY_HEADER,
        HeaderValue::from_static(release.stability.as_str()),
    );
    Ok(response)
}
