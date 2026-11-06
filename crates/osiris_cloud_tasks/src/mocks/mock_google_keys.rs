//! Google's signing keys faked: tokens signed by a key generated for the test, so no private key is
//! kept in the repository, and an endpoint that publishes the keys.

#![allow(
    clippy::unwrap_used,
    clippy::missing_panics_doc,
    reason = "a mock: a failure here is the test failing"
)]

use aws_lc_rs::{
    rand::SystemRandom,
    rsa::{KeyPair, KeySize, PublicKeyComponents},
    signature::{KeyPair as _, RSA_PKCS1_SHA256},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde_json::{Value, json};
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

pub struct MockGoogleSigner {
    key: KeyPair,
    kid: &'static str,
}

impl MockGoogleSigner {
    pub fn new(kid: &'static str) -> Self {
        Self {
            key: KeyPair::generate(KeySize::Rsa2048).unwrap(),
            kid,
        }
    }

    pub fn jwk(&self) -> Value {
        let public = PublicKeyComponents::<Vec<u8>>::from(self.key.public_key());
        json!({ "kty": "RSA", "alg": "RS256", "use": "sig", "kid": self.kid,
                "n": URL_SAFE_NO_PAD.encode(&public.n), "e": URL_SAFE_NO_PAD.encode(&public.e) })
    }

    /// An `Authorization` header value carrying `claims`, signed by this key.
    pub fn token(&self, claims: &Value) -> String {
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

/// Makes the server publish `signers`' keys, whatever is asked.
pub async fn answer_google_keys(server: &MockServer, signers: &[&MockGoogleSigner]) {
    let keys: Vec<Value> = signers.iter().map(|signer| signer.jwk()).collect();
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "keys": keys })))
        .mount(server)
        .await;
}

/// Makes the server fail every request for the keys.
pub async fn answer_google_keys_down(server: &MockServer) {
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .mount(server)
        .await;
}
