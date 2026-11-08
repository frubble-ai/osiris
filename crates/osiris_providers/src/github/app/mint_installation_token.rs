//! An installation token: the app's JWT exchanged for a token that acts as one installation, with
//! the permissions the installation granted. GitHub's tokens live an hour and can't be refreshed,
//! only minted again.

use super::{AppCredentials, app_jwt::app_jwt};
use crate::{
    InstallationToken, ProviderError,
    github::{Github, path_segment::path_segment, request::send},
};
use reqwest::Method;
use serde::Deserialize;

#[derive(Deserialize)]
struct MintedAnswer {
    token: String,
    expires_at: String,
}

impl Github {
    /// `POST /app/installations/{installation_id}/access_tokens`, as the app.
    pub async fn mint_installation_token(
        &self,
        http: &reqwest::Client,
        credentials: &AppCredentials,
        installation_id: &str,
    ) -> Result<InstallationToken, ProviderError> {
        let jwt = app_jwt(credentials)?;
        let path = format!(
            "/app/installations/{}/access_tokens",
            path_segment(installation_id)
        );
        let minted: MintedAnswer = send(
            self,
            http,
            &jwt,
            Method::POST,
            &path,
            &[],
            Some(&serde_json::json!({})),
        )
        .await?;
        Ok(InstallationToken {
            token: minted.token,
            expires_at: minted.expires_at,
        })
    }
}
