//! Slack's provider: its OAuth definition, its URLs, and how it ends a grant.

use crate::{ListLimits, OAuthClient, Provider, ProviderCommon, ProviderError, ProviderUrls};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use url::Url;

pub struct Slack {
    pub(super) oauth: OAuth2Definition,
    pub(super) api_base: Url,
    pub(super) limits: ListLimits,
}

impl Slack {
    /// `api_base` ends with `/` (`https://slack.com/api/`): methods join onto it.
    pub fn new(urls: &ProviderUrls, limits: ListLimits) -> Result<Self, url::ParseError> {
        Ok(Self {
            oauth: OAuth2Definition {
                authorization_url: urls.oauth_base.join("/oauth/v2_user/authorize")?,
                token_url: urls.api_base.join("oauth.v2.user.access")?,
                refresh_token_url: Some(urls.api_base.join("oauth.v2.access")?),
                scope_separator: ",",
                client_auth: ClientAuth::Post,
                pkce: false,
                authorization_params: &[],
                // Slack issues refresh tokens when the app has token rotation turned on.
                issues_refresh_tokens: true,
            },
            api_base: urls.api_base.clone(),
            limits,
        })
    }
}

impl ProviderCommon for Slack {
    fn id(&self) -> &'static str {
        "slack"
    }

    fn base_url(&self) -> &Url {
        &self.api_base
    }
}

#[async_trait]
impl Provider for Slack {
    fn oauth(&self) -> &OAuth2Definition {
        &self.oauth
    }

    async fn revoke(
        &self,
        http: &reqwest::Client,
        _client: &OAuthClient<'_>,
        access_token: &str,
    ) -> Result<(), ProviderError> {
        super::revoke::revoke(self, http, access_token).await
    }
}
