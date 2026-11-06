//! PKCE (RFC 7636): the verifier osiris keeps, and the S256 challenge it sends.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

/// A fresh PKCE verifier (RFC 7636): 48 random bytes, base64url.
pub fn pkce_verifier() -> String {
    let mut bytes = [0u8; 48];
    rand::fill(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub(super) fn pkce_challenge(verifier: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
}

#[cfg(test)]
mod tests;
