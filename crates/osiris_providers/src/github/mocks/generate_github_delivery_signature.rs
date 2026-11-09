//! A webhook delivery's signature, as GitHub signs it with the webhook's secret.

use aws_lc_rs::hmac;

/// The `X-Hub-Signature-256` header GitHub sends with `body`, signed with `secret`.
pub fn generate_github_delivery_signature(body: &[u8], secret: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    format!("sha256={}", hex::encode(hmac::sign(&key, body).as_ref()))
}
