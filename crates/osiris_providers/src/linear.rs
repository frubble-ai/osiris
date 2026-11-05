//! Linear: OAuth for a user, and reads through its GraphQL API. Endpoints as in `OpenConnector`
//! v1.9.4 (Apache-2.0), `src/providers/linear/definition.ts`.

pub mod graphql;
pub mod list_teams;
pub mod revoke;
pub mod transformers;
pub mod types;

use crate::{ListLimits, OAuthClient, Provider, ProviderCommon, ProviderError, ProviderUrls};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use url::Url;

pub struct Linear {
    oauth: OAuth2Definition,
    api_base: Url,
    limits: ListLimits,
}

impl Linear {
    pub fn new(urls: &ProviderUrls, limits: ListLimits) -> Result<Self, url::ParseError> {
        Ok(Self {
            oauth: OAuth2Definition {
                authorization_url: urls.oauth_base.join("/oauth/authorize")?,
                token_url: urls.api_base.join("/oauth/token")?,
                refresh_token_url: None,
                scope_separator: ",",
                client_auth: ClientAuth::Post,
                pkce: false,
                authorization_params: &[("prompt", "consent")],
                // Linear issues refresh tokens with every token.
                issues_refresh_tokens: true,
            },
            api_base: urls.api_base.clone(),
            limits,
        })
    }
}

impl ProviderCommon for Linear {
    fn id(&self) -> &'static str {
        "linear"
    }

    fn base_url(&self) -> &Url {
        &self.api_base
    }
}

#[async_trait]
impl Provider for Linear {
    fn oauth(&self) -> &OAuth2Definition {
        &self.oauth
    }

    async fn revoke(
        &self,
        http: &reqwest::Client,
        _client: &OAuthClient<'_>,
        access_token: &str,
    ) -> Result<(), ProviderError> {
        revoke::revoke(self, http, access_token).await
    }
}
