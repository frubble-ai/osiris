//! The consent's code, exchanged at the provider for tokens.

use crate::core::{
    Core, CoreResult, errors::provider as provider_error, oauth_client::oauth_client,
    refresh::new_tokens::new_tokens,
};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::{ConnectedAccount, DomainError};
use osiris_interfaces::oauth;
use osiris_storage::connected_accounts::NewTokens;

impl Core {
    /// The tokens, or the report of why there are none.
    pub(crate) async fn exchange(
        &self,
        account: &ConnectedAccount,
        code: &str,
        verifier: Option<&str>,
    ) -> CoreResult<NewTokens> {
        let config = self
            .store
            .auth_config(&account.auth_config_id)
            .await
            .switch()?;
        let provider = self
            .providers
            .get(&config.provider)
            .ok_or(DomainError::Internal)?;
        let redirect_uri = self.redirect_uri();
        oauth::exchange_code(
            provider.id(),
            &self.http,
            provider.oauth(),
            &oauth_client(&config, &redirect_uri),
            code,
            verifier,
        )
        .await
        .map(new_tokens)
        .map_err(provider_error)
    }
}
