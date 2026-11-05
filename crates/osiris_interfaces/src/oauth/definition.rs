//! A provider's `OAuth2` endpoints and conventions: data, not code. Ported from `OpenConnector` v1.9.4
//! (Apache-2.0, oomol-lab/open-connector), `OAuth2AuthDefinition` in `src/core/types.ts`.

use url::Url;

/// How a provider's token endpoint expects the client to authenticate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClientAuth {
    /// Client id and secret in the `Authorization: Basic` header.
    Basic,
    /// Client id and secret as form fields.
    Post,
}

/// A provider's `OAuth2` endpoints and conventions. Data, not code: the flow below runs any of them.
#[derive(Clone, Debug)]
pub struct OAuth2Definition {
    pub authorization_url: Url,
    pub token_url: Url,
    /// Where a refresh goes, when not the token URL.
    pub refresh_token_url: Option<Url>,
    /// How scopes are joined in the consent URL: `" "` (the RFC's) or `","`.
    pub scope_separator: &'static str,
    pub client_auth: ClientAuth,
    /// Send a PKCE S256 challenge with the consent, and its verifier with the code.
    pub pkce: bool,
    /// Fixed extra parameters on the consent URL.
    pub authorization_params: &'static [(&'static str, &'static str)],
    /// Whether the provider issues refresh tokens. When it does, a connection is only active once
    /// it holds a refresh token and its scheduled refresh; a consent that returns no refresh token
    /// is not activated. When it does not (for example, Trello's tokens never expire), a
    /// connection is complete with its access token alone and nothing is scheduled.
    pub issues_refresh_tokens: bool,
}
