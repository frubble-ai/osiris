//! Acme: OAuth for a user, and reads with the token. Copy this folder to
//! `crates/osiris_providers/src/`, rename `acme`/`Acme`, then follow docs/add_provider.md.

pub mod list_widgets;
pub mod revoke;
pub mod transformers;
pub mod types;

use crate::{OAuthClient, Provider, ProviderCommon, ProviderError, ProviderUrls};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use url::Url;

pub struct Acme {
    oauth: OAuth2Definition,
    api_base: Url,
}

impl Acme {
    pub fn new(urls: &ProviderUrls) -> Result<Self, url::ParseError> {
        Ok(Self {
            oauth: OAuth2Definition {
                // From the provider's OAuth docs.
                authorization_url: urls.oauth_base.join("/oauth/authorize")?,
                token_url: urls.api_base.join("/oauth/token")?,
                refresh_token_url: None,
                scope_separator: " ",
                client_auth: ClientAuth::Post,
                pkce: true,
                authorization_params: &[],
                // Whether Acme issues refresh tokens; false for a provider whose tokens never expire.
                issues_refresh_tokens: true,
            },
            api_base: urls.api_base.clone(),
        })
    }
}

impl ProviderCommon for Acme {
    fn id(&self) -> &'static str {
        "acme"
    }

    fn base_url(&self) -> &Url {
        &self.api_base
    }

    // Override `auth_header` or `parse_error` only when the provider differs from a bearer token
    // and status-coded errors.
}

#[async_trait]
impl Provider for Acme {
    fn oauth(&self) -> &OAuth2Definition {
        &self.oauth
    }

    async fn revoke(&self, http: &reqwest::Client, _client: &OAuthClient<'_>, access_token: &str) -> Result<(), ProviderError> {
        revoke::revoke(self, http, access_token).await
    }
}
