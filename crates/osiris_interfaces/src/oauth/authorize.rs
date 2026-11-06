//! The provider's consent URL.

use super::{definition::OAuth2Definition, pkce::pkce_challenge};
use crate::oauth_client::OAuthClient;
use url::Url;

/// The provider's consent page for `client`, bringing `state` back to osiris's callback.
pub fn authorize_url(
    definition: &OAuth2Definition,
    client: &OAuthClient<'_>,
    state: &str,
    pkce_verifier: Option<&str>,
) -> Url {
    let mut url = definition.authorization_url.clone();
    {
        let mut query = url.query_pairs_mut();
        query
            .append_pair("response_type", "code")
            .append_pair("client_id", client.client_id)
            .append_pair("redirect_uri", client.redirect_uri)
            .append_pair("state", state);
        if !client.scopes.is_empty() {
            query.append_pair("scope", &client.scopes.join(definition.scope_separator));
        }
        if let (true, Some(verifier)) = (definition.pkce, pkce_verifier) {
            query
                .append_pair("code_challenge", &pkce_challenge(verifier))
                .append_pair("code_challenge_method", "S256");
        }
        for (name, value) in definition.authorization_params {
            query.append_pair(name, value);
        }
    }
    url
}

#[cfg(test)]
mod tests;
