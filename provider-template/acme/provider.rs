//! Acme's provider: its OAuth definition, its URLs, and how it ends a grant.

use crate::{ListLimits, OAuthClient, Provider, ProviderCommon, ProviderError, ProviderUrls};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use url::Url;

pub struct Acme {
    oauth: OAuth2Definition,
    api_base: Url,
    /// How far its lists go (`[providers.listing]`).
    pub(super) limits: ListLimits,
}

impl Acme {
    pub fn new(urls: &ProviderUrls, limits: ListLimits) -> Result<Self, url::ParseError> {
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
            limits,
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

    // Override `auth_header`, `parse_error` or `scope_covers` only when the provider differs
    // from a bearer token, errors read by their status, and scopes that don't nest.
}

#[async_trait]
impl Provider for Acme {
    fn oauth(&self) -> &OAuth2Definition {
        &self.oauth
    }

    async fn revoke(&self, http: &reqwest::Client, _client: &OAuthClient<'_>, access_token: &str) -> Result<(), ProviderError> {
        super::revoke::revoke(self, http, access_token).await
    }
}
