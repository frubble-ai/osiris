//! Tests of `bearer`.

use super::*;
use fake::{Fake, faker::internet::en::Password};
use osiris_common::secret::secret;

#[tokio::test]
async fn accepts_only_the_token() {
    let token: String = Password(32..40).fake();
    let auth = BearerAuthenticator::new(secret(&token)).unwrap();
    assert!(
        auth.authenticate(Some(&format!("Bearer {token}")))
            .await
            .is_ok()
    );
    assert!(auth.authenticate(Some("Bearer wrong")).await.is_err());
    assert!(auth.authenticate(Some(&token)).await.is_err());
    assert!(auth.authenticate(None).await.is_err());
}

#[test]
fn refuses_a_short_token() {
    assert!(BearerAuthenticator::new(secret("short")).is_err());
}
