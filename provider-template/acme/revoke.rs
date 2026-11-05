//! Acme ends a grant at its revoke endpoint. A token it no longer has counts as revoked.

use super::Acme;
use crate::{ProviderCommon, ProviderError};

pub async fn revoke(acme: &Acme, http: &reqwest::Client, access_token: &str) -> Result<(), ProviderError> {
    let url = acme.base_url().join("/oauth/revoke").map_err(|error| ProviderError::unavailable(acme.id(), error))?;
    let response = http.post(url).form(&[("token", access_token)]).send().await.map_err(|error| ProviderError::unavailable(acme.id(), error))?;
    match response.status().as_u16() {
        200 | 400 | 401 => Ok(()),
        status => Err(ProviderError::unavailable(acme.id(), format!("revoke answered {status}"))),
    }
}
