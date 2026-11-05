//! The OIDC authenticator against a fake key endpoint: tokens signed by a key generated for the
//! test, so no private key is kept in the repository.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use osiris_cloud_tasks::{
    OidcAuthenticator, OidcConfig,
    mocks::mock_google_keys::{MockGoogleSigner as Signer, answer_google_keys},
};
use osiris_tasks::TaskAuthenticator;
use rstest::rstest;
use serde_json::{Value, json};
use std::time::Duration;
use wiremock::MockServer;

const AUDIENCE: &str = "https://osiris-worker.example.run.app";
const ACCOUNT: &str = "tasks@project.iam.gserviceaccount.com";

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
    answer_google_keys(&server, keys).await;
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

#[rstest]
#[case::another_audience(vec![("aud", json!("https://someone-else.run.app"))])]
#[case::another_account(vec![("email", json!("other@project.iam.gserviceaccount.com"))])]
#[case::unverified_email(vec![("email_verified", json!(false))])]
#[case::another_issuer(vec![("iss", json!("https://evil.example"))])]
#[case::expired(vec![("exp", json!(1_000)), ("iat", json!(700))])]
#[tokio::test]
async fn refuses_a_token_that_is_not_for_the_worker_from_google(
    #[case] overrides: Vec<(&str, Value)>,
) {
    let google = Signer::new("k1");
    let (auth, _server) = authenticator(&[&google]).await;

    let token = google.token(&claims(&overrides));

    assert!(auth.authenticate(Some(&token)).await.is_err());
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
