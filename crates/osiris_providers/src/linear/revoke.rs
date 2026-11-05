//! Linear ends a grant at its revoke endpoint, with the token itself.

use super::Linear;
use crate::{ProviderCommon, ProviderError};

pub async fn revoke(
    linear: &Linear,
    http: &reqwest::Client,
    access_token: &str,
) -> Result<(), ProviderError> {
    let url = linear
        .base_url()
        .join("/oauth/revoke")
        .map_err(|error| ProviderError::unavailable(linear.id(), error))?;
    let response = http
        .post(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|error| ProviderError::unavailable(linear.id(), error))?;
    match response.status().as_u16() {
        // Revoked now, or a token Linear no longer accepts.
        200 | 400 | 401 => Ok(()),
        status => Err(ProviderError::unavailable(
            linear.id(),
            format!("revoke answered {status}"),
        )),
    }
}
