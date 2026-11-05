//! GitHub, through our GitHub App: OAuth for the app's user token, and reads with it. The token
//! reads only what the app is installed on, so the reads are the app's installations and the
//! repositories they give. OAuth endpoints as in `OpenConnector` v1.9.4 (Apache-2.0),
//! `src/providers/github/definition.ts`.

pub mod get_page;
pub mod list_installations;
pub mod list_repositories;
pub mod revoke;
pub mod transformers;
pub mod types;

use crate::{ListLimits, OAuthClient, Provider, ProviderCommon, ProviderError, ProviderUrls};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use url::Url;

pub struct Github {
    oauth: OAuth2Definition,
    api_base: Url,
    limits: ListLimits,
}

impl Github {
    pub fn new(urls: &ProviderUrls, limits: ListLimits) -> Result<Self, url::ParseError> {
        Ok(Self {
            oauth: OAuth2Definition {
                authorization_url: urls.oauth_base.join("/login/oauth/authorize")?,
                token_url: urls.oauth_base.join("/login/oauth/access_token")?,
                refresh_token_url: None,
                scope_separator: " ",
                client_auth: ClientAuth::Post,
                pkce: false,
                authorization_params: &[],
                // GitHub Apps issue refresh tokens when expiring user tokens are on (the default).
                issues_refresh_tokens: true,
            },
            api_base: urls.api_base.clone(),
            limits,
        })
    }
}

impl ProviderCommon for Github {
    fn id(&self) -> &'static str {
        "github"
    }

    fn base_url(&self) -> &Url {
        &self.api_base
    }
}

#[async_trait]
impl Provider for Github {
    fn oauth(&self) -> &OAuth2Definition {
        &self.oauth
    }

    async fn revoke(
        &self,
        http: &reqwest::Client,
        client: &OAuthClient<'_>,
        access_token: &str,
    ) -> Result<(), ProviderError> {
        revoke::revoke(self, http, client, access_token).await
    }
}
