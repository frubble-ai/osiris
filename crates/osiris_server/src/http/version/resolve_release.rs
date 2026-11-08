//! The release a request asks for: the one its `Osiris-Version` names, or the pinned one.

use crate::http::{error::ApiError, state::ApiState};
use axum::http::HeaderMap;
use osiris_versioning::{HEADER, Release};

/// The release `headers` name in `Osiris-Version`, the pinned one when they name none, or
/// `OS_07` for one osiris doesn't serve.
pub fn resolve_release(
    state: &ApiState,
    headers: &HeaderMap,
) -> Result<&'static Release, ApiError> {
    let asked = match headers.get(HEADER) {
        Some(value) => value
            .to_str()
            .map_err(|_| ApiError::UnknownVersion("(not text)".into()))?,
        None => state.config.settings.api.pinned_version.as_str(),
    };
    state
        .releases
        .iter()
        .find(|release| release.name == asked)
        .ok_or_else(|| ApiError::UnknownVersion(asked.to_owned()))
}
