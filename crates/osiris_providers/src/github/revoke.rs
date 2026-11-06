//! GitHub ends a grant at its own endpoint, with the app's credentials, not by RFC 7009.

use super::Github;
use crate::{OAuthClient, ProviderCommon, ProviderError};
use serde_json::json;

pub async fn revoke(
    github: &Github,
    http: &reqwest::Client,
    client: &OAuthClient<'_>,
    access_token: &str,
) -> Result<(), ProviderError> {
    let url = github
        .base_url()
        .join(&format!("/applications/{}/grant", client.client_id))
        .map_err(|error| ProviderError::unavailable(github.id(), error))?;
    let response = http
        .delete(url)
        .basic_auth(client.client_id, Some(client.client_secret))
        .header("accept", "application/vnd.github+json")
        .json(&json!({ "access_token": access_token }))
        .send()
        .await
        .map_err(|error| ProviderError::unavailable(github.id(), error))?;
    match response.status().as_u16() {
        // Revoked now, or a grant GitHub no longer has.
        204 | 404 | 422 => Ok(()),
        status => Err(ProviderError::unavailable(
            github.id(),
            format!("revoke answered {status}"),
        )),
    }
}
