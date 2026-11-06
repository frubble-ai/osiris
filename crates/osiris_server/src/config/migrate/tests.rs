//! Tests of `migrate`.

use super::reset_requested;
use rstest::rstest;

#[rstest]
#[case::exact(Some("true"), true)]
#[case::unset(None, false)]
#[case::empty(Some(""), false)]
#[case::one(Some("1"), false)]
#[case::upper(Some("TRUE"), false)]
#[case::false_(Some("false"), false)]
fn reset_is_only_the_exact_string_true(#[case] value: Option<&str>, #[case] reset: bool) {
    assert_eq!(reset_requested(value), reset);
}

#[rstest]
#[case::matches(true, Some("osiris"), "osiris", true)]
#[case::not_resetting(false, None, "osiris", true)]
#[case::missing(true, None, "osiris", false)]
#[case::blank(true, Some("  "), "osiris", false)]
#[case::other_database(true, Some("other"), "osiris", false)]
fn reset_names_the_connected_database(
    #[case] reset: bool,
    #[case] target: Option<&str>,
    #[case] connected: &str,
    #[case] allowed: bool,
) {
    assert_eq!(
        super::require_reset_target(reset, target, connected).is_ok(),
        allowed
    );
}
