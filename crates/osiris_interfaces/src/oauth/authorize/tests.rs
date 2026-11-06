//! Tests of `authorize`.

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
