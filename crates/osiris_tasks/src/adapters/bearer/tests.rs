//! Tests of `bearer`.

use super::*;
use osiris_common::secret::secret;

const TOKEN: &str = "a-worker-token-that-is-long-enough!";

#[tokio::test]
async fn accepts_only_the_token() {
    let auth = BearerAuthenticator::new(secret(TOKEN)).unwrap();
    assert!(
        auth.authenticate(Some(&format!("Bearer {TOKEN}")))
            .await
            .is_ok()
    );
    assert!(auth.authenticate(Some("Bearer wrong")).await.is_err());
    assert!(auth.authenticate(Some(TOKEN)).await.is_err());
    assert!(auth.authenticate(None).await.is_err());
}

#[test]
fn refuses_a_short_token() {
    assert!(BearerAuthenticator::new(secret("short")).is_err());
}
