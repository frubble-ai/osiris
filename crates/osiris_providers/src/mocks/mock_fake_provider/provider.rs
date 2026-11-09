//! The fake provider: its OAuth definition, its URLs, and how it ends a grant.

use crate::{
    ListLimits, OAuthClient, Provider, ProviderCommon, ProviderError, ProviderUrls, TriggerCheck,
};
use async_trait::async_trait;
use osiris_interfaces::oauth::{ClientAuth, OAuth2Definition};
use serde_json::{Value, json};
use url::Url;

/// Its id: a row the tests add to the `provider` table.
pub const FAKE_ID: &str = "fake";

pub struct FakeProvider {
    oauth: OAuth2Definition,
    api_base: Url,
}

impl FakeProvider {
    pub fn new(urls: &ProviderUrls, _limits: ListLimits) -> Result<Self, url::ParseError> {
        Ok(Self {
            oauth: OAuth2Definition {
                authorization_url: urls.oauth_base.join("/fake/oauth/authorize")?,
                token_url: urls.api_base.join("/fake/oauth/token")?,
                refresh_token_url: None,
                scope_separator: " ",
                client_auth: ClientAuth::Post,
                pkce: false,
                authorization_params: &[],
                issues_refresh_tokens: true,
            },
            api_base: urls.api_base.clone(),
        })
    }
}

impl ProviderCommon for FakeProvider {
    fn id(&self) -> &'static str {
        FAKE_ID
    }

    fn base_url(&self) -> &Url {
        &self.api_base
    }
}

#[async_trait]
impl Provider for FakeProvider {
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

    /// Its triggers check on a schedule and see nothing new, unless their config holds
    /// `"unreadable": true`, which it refuses as a config it can't take.
    fn trigger_checks_on_schedule(&self, _kind: &str) -> bool {
        true
    }

    async fn check_trigger(
        &self,
        _http: &reqwest::Client,
        _token: &str,
        _kind: &str,
        config: &Value,
        checkpoint: Option<&Value>,
    ) -> Result<TriggerCheck, ProviderError> {
        if config.get("unreadable").is_some() {
            return Err(ProviderError::invalid(
                FAKE_ID,
                "the trigger's config doesn't read",
            ));
        }
        Ok(TriggerCheck {
            checkpoint: checkpoint.cloned().unwrap_or_else(|| json!({})),
            fired: None,
        })
    }
}
