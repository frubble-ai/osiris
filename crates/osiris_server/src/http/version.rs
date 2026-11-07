//! `Osiris-Version`: a request names the version it was written for; one osiris doesn't serve is
//! refused, and one naming none is served at the API key's pinned version (`[api]`). Every answer
//! names the version it was served at.

use super::{error::ApiError, state::ApiState};
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use osiris_api_models::version::{HEADER, find};

pub async fn version(
    State(state): State<ApiState>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let asked = match request.headers().get(HEADER) {
        Some(value) => value
            .to_str()
            .map_err(|_| ApiError::UnknownVersion("(not text)".into()))?
            .to_owned(),
        None => state.config.settings.api.pinned_version.clone(),
    };
    let served = find(&asked).ok_or(ApiError::UnknownVersion(asked))?.name;
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(HEADER, HeaderValue::from_static(served));
    Ok(response)
}
