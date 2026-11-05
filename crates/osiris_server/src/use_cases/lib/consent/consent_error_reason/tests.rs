//! Tests of `consent_error_reason`.

use super::*;
use rstest::rstest;

#[test]
fn names_a_standard_error_code() {
    assert_eq!(
        consent_error_reason("access_denied"),
        "the provider answered access_denied"
    );
}

#[rstest]
#[case::script_tag("<script>".to_owned())]
#[case::near_miss_code("access_denied_or_not".to_owned())]
#[case::oversized("x".repeat(10_000))]
fn names_nothing_else(#[case] sent: String) {
    assert_eq!(
        consent_error_reason(&sent),
        "the provider refused the consent"
    );
}
