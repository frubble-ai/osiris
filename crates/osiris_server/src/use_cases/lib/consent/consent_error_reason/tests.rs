//! Tests of `consent_error_reason`.

use super::*;

#[test]
fn names_only_the_standard_error_codes() {
    assert_eq!(
        consent_error_reason("access_denied"),
        "the provider answered access_denied"
    );
    for sent in ["<script>", "access_denied_or_not", &"x".repeat(10_000)] {
        assert_eq!(
            consent_error_reason(sent),
            "the provider refused the consent"
        );
    }
}
