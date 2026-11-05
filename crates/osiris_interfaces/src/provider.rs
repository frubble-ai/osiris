//! A provider osiris connects: what every provider is, its `OAuth2` definition, and how it ends a
//! grant.

use crate::{OAuthClient, ProviderCommon, ProviderError, oauth::OAuth2Definition};
use async_trait::async_trait;

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
}
