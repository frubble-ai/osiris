//! Tests of `create`.

use super::*;
use fake::{Fake, faker::internet::en::Password};

#[test]
fn sends_the_app_with_its_scopes() {
    let client_id: String = Password(8..16).fake();
    let client_secret: String = Password(32..40).fake();
    let create = Create {
        provider: "acme".into(),
        client_id: client_id.clone(),
        client_secret: client_secret.clone(),
        scopes: vec!["read".into()],
    };

    assert_eq!(
        create.body(),
        json!({ "provider": "acme", "client_id": client_id, "client_secret": client_secret, "scopes": ["read"] })
    );
}
