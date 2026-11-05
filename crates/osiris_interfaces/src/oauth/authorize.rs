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
mod tests {
    use super::*;
    use crate::oauth::{
        definition::ClientAuth,
        pkce::{pkce_challenge, pkce_verifier},
    };

    fn definition(pkce: bool) -> OAuth2Definition {
        OAuth2Definition {
            authorization_url: Url::parse("https://provider.test/authorize").unwrap(),
            token_url: Url::parse("https://provider.test/token").unwrap(),
            refresh_token_url: None,
            scope_separator: ",",
            client_auth: ClientAuth::Post,
            pkce,
            authorization_params: &[("prompt", "consent")],
            issues_refresh_tokens: true,
        }
    }

    fn client(scopes: &[String]) -> OAuthClient<'_> {
        OAuthClient {
            client_id: "client",
            client_secret: "secret",
            scopes,
            redirect_uri: "https://osiris.test/oauth/callback",
        }
    }

    #[test]
    fn builds_the_consent_url_with_scopes_state_pkce_and_fixed_params() {
        let scopes = vec!["read".to_owned(), "write".to_owned()];
        let verifier = pkce_verifier();
        let url = authorize_url(
            &definition(true),
            &client(&scopes),
            "the-state",
            Some(&verifier),
        );
        let query: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(query["scope"], "read,write");
        assert_eq!(query["state"], "the-state");
        assert_eq!(query["prompt"], "consent");
        assert_eq!(query["code_challenge"], pkce_challenge(&verifier));
        assert_eq!(query["code_challenge_method"], "S256");
    }

    #[test]
    fn leaves_pkce_out_for_a_provider_without_it() {
        let url = authorize_url(&definition(false), &client(&[]), "s", Some("verifier"));
        assert!(!url.query_pairs().any(|(name, _)| name == "code_challenge"));
        assert!(!url.query_pairs().any(|(name, _)| name == "scope"));
    }
}
