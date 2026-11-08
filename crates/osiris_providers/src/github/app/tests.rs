//! Our GitHub App's credentials read from an auth config's settings, its JWT checked against the
//! app's public key, and an installation token minted with it.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{AppCredentials, app_jwt::app_jwt};
use crate::github::mocks::{
    generate_github::generate_github, mock_github_app_key::MockGithubAppKey,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use osiris_common::secret::ExposeSecret;
use rstest::rstest;
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header_regex, method, path},
};

#[derive(serde::Deserialize)]
struct Claims {
    iss: String,
    iat: u64,
    exp: u64,
}

#[test]
fn reads_credentials_with_an_escaped_key() {
    let key = MockGithubAppKey::default().pem().replace('\n', "\\n");
    let credentials = AppCredentials::read(
        &json!({ "app_id": " 12345 " }),
        Some(&json!({ "private_key": key })),
    )
    .unwrap();

    assert_eq!(credentials.app_id, "12345");
    assert!(
        credentials
            .private_key
            .starts_with(concat!("-----BEGIN ", "PRIVATE KEY-----\n"))
    );
}

/// A PEM's first line with no key after it, split so the secret scan doesn't read it as a key.
const PEM_HEADER_ONLY: &str = concat!("-----BEGIN RSA ", "PRIVATE KEY-----");

#[rstest]
#[case::no_app_id(json!({}), json!({ "private_key": PEM_HEADER_ONLY }), "app_id")]
#[case::an_app_id_with_a_slash(json!({ "app_id": "1/2" }), json!({ "private_key": PEM_HEADER_ONLY }), "app_id")]
#[case::no_key(json!({ "app_id": "1" }), json!({}), "private_key")]
#[case::not_a_pem(json!({ "app_id": "1" }), json!({ "private_key": "hunter2" }), "private_key")]
fn names_what_the_settings_lack(
    #[case] settings: Value,
    #[case] secret_settings: Value,
    #[case] named: &str,
) {
    let Err(reason) = AppCredentials::read(&settings, Some(&secret_settings)) else {
        panic!("the settings were accepted");
    };
    assert!(reason.contains(named), "{reason}");
}

#[test]
fn signs_a_jwt_the_app_s_public_key_verifies_issued_by_the_app() {
    let key = MockGithubAppKey::default();
    let credentials = AppCredentials {
        app_id: "12345".into(),
        private_key: key.pem(),
    };

    let jwt = app_jwt(&credentials).unwrap();

    let (n, e) = key.public_components();
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["iss", "exp", "iat"]);
    validation.set_issuer(&["12345"]);
    let claims = decode::<Claims>(
        &jwt,
        &DecodingKey::from_rsa_components(&n, &e).unwrap(),
        &validation,
    )
    .unwrap()
    .claims;
    assert_eq!(claims.iss, "12345");
    // Nine minutes, under GitHub's ten-minute limit.
    assert_eq!(claims.exp - claims.iat, 9 * 60);
}

#[tokio::test]
async fn mints_an_installation_token_as_the_app() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/app/installations/678/access_tokens"))
        .and(header_regex(
            "authorization",
            r"^Bearer [\w-]+\.[\w-]+\.[\w-]+$",
        ))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "token": "ghs_installation",
            "expires_at": "2026-11-08T04:00:00Z",
            "permissions": { "contents": "read" },
        })))
        .expect(1)
        .mount(&server)
        .await;
    let credentials = AppCredentials {
        app_id: "12345".into(),
        private_key: MockGithubAppKey::default().pem(),
    };

    let minted = generate_github(&server)
        .mint_installation_token(&reqwest::Client::new(), &credentials, "678")
        .await
        .unwrap();

    assert_eq!(minted.token.expose_secret(), "ghs_installation");
    assert_eq!(minted.expires_at, "2026-11-08T04:00:00Z");
}
