//! Tests of PKCE against RFC 7636.

use super::{pkce_challenge, pkce_verifier};

#[test]
fn challenges_the_rfc_7636_appendix_b_verifier() {
    // RFC 7636 Appendix B.
    let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
    assert_eq!(
        pkce_challenge(verifier),
        "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
    );
}

#[test]
fn verifiers_are_base64url_of_the_length_rfc_7636_allows() {
    let verifier = pkce_verifier();
    assert!((43..=128).contains(&verifier.len()));
    assert!(
        verifier
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'),
        "{verifier}"
    );
}
