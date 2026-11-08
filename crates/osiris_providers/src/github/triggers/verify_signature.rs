//! Whether a delivery was signed with the trigger's secret: GitHub's `X-Hub-Signature-256`,
//! `sha256=` and the hex HMAC-SHA256 of the raw body, compared in constant time.

use aws_lc_rs::hmac;

/// Whether `signature` (the header's value) signs `body` with `secret`.
pub fn verify_signature(secret: &str, body: &[u8], signature: Option<&str>) -> bool {
    let Some(hex_digest) = signature.and_then(|value| value.strip_prefix("sha256=")) else {
        return false;
    };
    let Ok(tag) = hex::decode(hex_digest) else {
        return false;
    };
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    hmac::verify(&key, body, &tag).is_ok()
}
