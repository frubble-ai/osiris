//! A provider osiris connects: what every provider is, its `OAuth2` definition, how it ends a
//! grant, the settings its apps may carry, for a provider whose apps are installed how an
//! installation's token is minted, and for a provider with triggers how one is subscribed to, how
//! its deliveries are read, and how it is checked on a schedule. A provider without them refuses.

use crate::no_settings::no_settings;
use crate::{
    InstallationToken, OAuthClient, ProviderCommon, ProviderError, TriggerCheck, TriggerReceipt,
    oauth::OAuth2Definition,
};
use async_trait::async_trait;
use reqwest::header::HeaderMap;
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
        if no_settings(settings, secret_settings) {
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

    /// Whether `config` configures a trigger of kind `kind` this provider has; the reason it
    /// doesn't otherwise.
    fn validate_trigger(&self, kind: &str, _config: &Value) -> Result<(), String> {
        Err(format!("{} has no trigger {kind:?}", self.id()))
    }

    /// Whether a trigger of kind `kind` also checks the provider on a schedule.
    fn trigger_checks_on_schedule(&self, _kind: &str) -> bool {
        false
    }

    /// Subscribes at the provider: deliveries for the trigger go to `callback_url`, signed with
    /// `secret`. Answers the provider's id of the subscription, if it has one.
    async fn subscribe_trigger(
        &self,
        _http: &reqwest::Client,
        _token: &str,
        _kind: &str,
        _config: &Value,
        _callback_url: &str,
        _secret: &str,
    ) -> Result<Option<String>, ProviderError> {
        Err(ProviderError::refused(
            self.id(),
            "this provider has no triggers",
        ))
    }

    /// Ends the subscription `subscription_id` at the provider. One already gone counts as ended.
    async fn unsubscribe_trigger(
        &self,
        _http: &reqwest::Client,
        _token: &str,
        _kind: &str,
        _config: &Value,
        _subscription_id: &str,
    ) -> Result<(), ProviderError> {
        Ok(())
    }

    /// What a delivery (its `headers` and raw `body`) means for a trigger of kind `kind`, signed
    /// with `secret`. A body longer than `max_data_bytes` can't be carried in an event, so a
    /// provider doesn't read it into the event's data; it notes its size instead.
    fn receive_trigger(
        &self,
        _kind: &str,
        _config: &Value,
        _headers: &HeaderMap,
        _body: &[u8],
        _secret: &str,
        _max_data_bytes: usize,
    ) -> TriggerReceipt {
        TriggerReceipt::Unsigned
    }

    /// Checks a trigger of kind `kind` at the provider, against what the last check saw.
    async fn check_trigger(
        &self,
        _http: &reqwest::Client,
        _token: &str,
        _kind: &str,
        _config: &Value,
        _checkpoint: Option<&Value>,
    ) -> Result<TriggerCheck, ProviderError> {
        Err(ProviderError::refused(
            self.id(),
            "this provider has no triggers",
        ))
    }
}
