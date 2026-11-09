//! A GitHub App's private key generated for the test, so no key is kept in the repository: its
//! PEM, as the app's settings page downloads one, and its public half, to check what it signed.

#![allow(
    clippy::unwrap_used,
    clippy::missing_panics_doc,
    reason = "a mock: a failure here is the test failing"
)]

use aws_lc_rs::{
    encoding::AsDer,
    rsa::{KeyPair, KeySize, PublicKeyComponents},
    signature::KeyPair as _,
};
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};

pub struct MockGithubAppKey {
    key: KeyPair,
}

impl Default for MockGithubAppKey {
    fn default() -> Self {
        Self {
            key: KeyPair::generate(KeySize::Rsa2048).unwrap(),
        }
    }
}

impl MockGithubAppKey {
    /// The private key in PKCS#8 PEM.
    pub fn pem(&self) -> String {
        let der = self.key.as_der().unwrap();
        let body = STANDARD.encode(der.as_ref());
        let lines: Vec<&str> = body
            .as_bytes()
            .chunks(64)
            .map(|line| std::str::from_utf8(line).unwrap())
            .collect();
        format!(
            "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
            lines.join("\n")
        )
    }

    /// The public key's modulus and exponent, base64url, as a JWT library reads them.
    pub fn public_components(&self) -> (String, String) {
        let public = PublicKeyComponents::<Vec<u8>>::from(self.key.public_key());
        (
            URL_SAFE_NO_PAD.encode(&public.n),
            URL_SAFE_NO_PAD.encode(&public.e),
        )
    }
}
