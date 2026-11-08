//! The fake provider ends a grant at its revoke endpoint, with the token itself.

use super::FakeProvider;
use crate::{ProviderCommon, ProviderError};

pub async fn revoke(
    fake: &FakeProvider,
    http: &reqwest::Client,
    access_token: &str,
) -> Result<(), ProviderError> {
    let url = fake
        .base_url()
        .join("/fake/oauth/revoke")
        .map_err(|error| ProviderError::unavailable(fake.id(), error))?;
    let response = http
        .post(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|error| ProviderError::unavailable(fake.id(), error))?;
    match response.status().as_u16() {
        // Revoked now, or a token it no longer accepts.
        200 | 400 | 401 => Ok(()),
        status => Err(ProviderError::unavailable(
            fake.id(),
            format!("revoke answered {status}"),
        )),
    }
}
