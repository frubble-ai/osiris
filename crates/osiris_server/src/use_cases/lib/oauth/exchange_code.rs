//! Exchanges a consent's code at the provider for tokens.

use super::{new_tokens::new_tokens, oauth_client::oauth_client};
use crate::use_cases::lib::connected_accounts::NewTokens;
use crate::use_cases::lib::shared::domain_errors::{UseCaseResult, provider as provider_error};
use osiris_domain::{AuthConfig, DomainError};
use osiris_interfaces::oauth;
use osiris_providers::Providers;

pub struct Exchange<'a> {
    pub config: &'a AuthConfig,
    pub code: &'a str,
    pub verifier: Option<&'a str>,
    pub redirect_uri: &'a str,
}

pub async fn exchange_code(
    providers: &Providers,
    http: &reqwest::Client,
    exchange: Exchange<'_>,
) -> UseCaseResult<NewTokens> {
    let provider = providers
        .get(&exchange.config.provider)
        .ok_or(DomainError::Internal)?;
    oauth::exchange_code(
        provider.id(),
        http,
        provider.oauth(),
        &oauth_client(exchange.config, exchange.redirect_uri),
        exchange.code,
        exchange.verifier,
    )
    .await
    .map(new_tokens)
    .map_err(provider_error)
}
