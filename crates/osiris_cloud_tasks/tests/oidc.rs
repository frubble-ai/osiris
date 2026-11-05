//! The OIDC authenticator against a fake key endpoint: tokens signed by a key generated for the
//! test, so no private key is kept in the repository.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use aws_lc_rs::{
    rand::SystemRandom,
    rsa::{KeyPair, KeySize, PublicKeyComponents},
    signature::{KeyPair as _, RSA_PKCS1_SHA256},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use osiris_cloud_tasks::{OidcAuthenticator, OidcConfig};
use osiris_tasks::TaskAuthenticator;
use serde_json::{Value, json};
use std::time::Duration;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

const AUDIENCE: &str = "https://osiris-worker.example.run.app";
const ACCOUNT: &str = "tasks@project.iam.gserviceaccount.com";

struct Signer {
    key: KeyPair,
    kid: &'static str,
}

impl Signer {
    fn new(kid: &'static str) -> Self {
        Self {
            key: KeyPair::generate(KeySize::Rsa2048).unwrap(),
            kid,
        }
    }

    fn jwk(&self) -> Value {
        let public = PublicKeyComponents::<Vec<u8>>::from(self.key.public_key());
        json!({ "kty": "RSA", "alg": "RS256", "use": "sig", "kid": self.kid,
                "n": URL_SAFE_NO_PAD.encode(&public.n), "e": URL_SAFE_NO_PAD.encode(&public.e) })
    }

    fn token(&self, claims: &Value) -> String {
        let header = json!({ "alg": "RS256", "typ": "JWT", "kid": self.kid });
        let signed = format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(header.to_string()),
            URL_SAFE_NO_PAD.encode(claims.to_string())
        );
        let mut signature = vec![0; self.key.public_modulus_len()];
        self.key
            .sign(
                &RSA_PKCS1_SHA256,
                &SystemRandom::new(),
                signed.as_bytes(),
                &mut signature,
            )
            .unwrap();
        format!("Bearer {signed}.{}", URL_SAFE_NO_PAD.encode(signature))
    }
}

fn claims(overrides: &[(&str, Value)]) -> Value {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let mut claims = json!({ "iss": "https://accounts.google.com", "aud": AUDIENCE, "email": ACCOUNT,
                             "email_verified": true, "iat": now, "exp": now + 300 });
    for (name, value) in overrides {
        claims[*name] = value.clone();
    }
    claims
}

async fn authenticator(keys: &[&Signer]) -> (OidcAuthenticator, MockServer) {
    let server = MockServer::start().await;
    let keys: Vec<Value> = keys.iter().map(|signer| signer.jwk()).collect();
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "keys": keys })))
        .mount(&server)
        .await;
    let authenticator = OidcAuthenticator::new(
        OidcConfig {
            keys_url: server.uri(),
            audience: AUDIENCE.into(),
            service_account_email: ACCOUNT.into(),
            keys_ttl: Duration::from_secs(3600),
            refetch_interval: Duration::from_secs(30),
        },
        reqwest::Client::new(),
    );
    (authenticator, server)
}

#[tokio::test]
async fn accepts_a_google_token_for_the_worker_and_the_service_account() {
    let google = Signer::new("k1");
    let (auth, _server) = authenticator(&[&google]).await;
    auth.authenticate(Some(&google.token(&claims(&[]))))
        .await
        .unwrap();
}

#[tokio::test]
async fn refuses_a_token_for_another_audience_account_issuer_or_an_expired_one() {
    let google = Signer::new("k1");
    let (auth, _server) = authenticator(&[&google]).await;
    let past = time::OffsetDateTime::now_utc().unix_timestamp() - 3600;
    for overrides in [
        vec![("aud", json!("https://someone-else.run.app"))],
        vec![("email", json!("other@project.iam.gserviceaccount.com"))],
        vec![("email_verified", json!(false))],
        vec![("iss", json!("https://evil.example"))],
        vec![("exp", json!(past)), ("iat", json!(past - 300))],
    ] {
        let token = google.token(&claims(&overrides));
        assert!(
            auth.authenticate(Some(&token)).await.is_err(),
            "{overrides:?}"
        );
    }
}

#[tokio::test]
async fn refuses_a_token_signed_by_a_key_google_did_not_publish_and_a_missing_token() {
    let google = Signer::new("k1");
    let forger = Signer::new("k1");
    let (auth, _server) = authenticator(&[&google]).await;
    assert!(
        auth.authenticate(Some(&forger.token(&claims(&[]))))
            .await
            .is_err()
    );
    let unknown_key = Signer::new("k9");
    assert!(
        auth.authenticate(Some(&unknown_key.token(&claims(&[]))))
            .await
            .is_err()
    );
    assert!(auth.authenticate(None).await.is_err());
    assert!(auth.authenticate(Some("Bearer not.a.token")).await.is_err());
}

#[tokio::test]
async fn fetches_google_s_keys_once_however_many_unknown_key_ids_it_is_sent() {
    let google = Signer::new("k1");
    let (auth, server) = authenticator(&[&google]).await;
    for kid in ["k2", "k3", "k4", "k5"] {
        assert!(
            auth.authenticate(Some(&Signer::new(kid).token(&claims(&[]))))
                .await
                .is_err()
        );
    }
    auth.authenticate(Some(&google.token(&claims(&[]))))
        .await
        .unwrap();

    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}
