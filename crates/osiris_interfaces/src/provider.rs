//! A provider osiris connects: what every provider is, its `OAuth2` definition, how it ends a
//! grant, the settings its apps may carry, and, for a provider whose apps are installed (GitHub),
//! how an installation's token is minted.

use crate::{
    InstallationToken, OAuthClient, ProviderCommon, ProviderError, oauth::OAuth2Definition,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Provider: ProviderCommon {
    fn oauth(&self) -> &OAuth2Definition;

    /// End the grant at the provider. A grant already gone counts as revoked.
    async fn revoke(
        &self,
        http: &reqwest::Client,
        client: &OAuthClient<'_>,
        access_token: &str,
    ) -> Result<(), ProviderError>;

    /// Whether an app's `settings` and `secret_settings` (both JSON objects) are ones this
    /// provider reads; the reason they aren't otherwise. A provider with no settings of its own
    /// accepts only none.
    fn validate_settings(
        &self,
        settings: &Value,
        secret_settings: Option<&Value>,
    ) -> Result<(), String> {
        let empty = |value: &Value| value.as_object().is_none_or(serde_json::Map::is_empty);
        if empty(settings) && secret_settings.is_none_or(empty) {
            Ok(())
        } else {
            Err(format!("{} takes no settings", self.id()))
        }
    }

    /// A token for the app's installation `installation_id`, minted with the app's `settings`
    /// and `secret_settings`. A provider whose apps aren't installed refuses.
    async fn installation_token(
        &self,
        _http: &reqwest::Client,
        _settings: &Value,
        _secret_settings: &Value,
        _installation_id: &str,
    ) -> Result<InstallationToken, ProviderError> {
        Err(ProviderError::refused(
            self.id(),
            "this provider's apps have no installations to act as",
        ))
    }
}
