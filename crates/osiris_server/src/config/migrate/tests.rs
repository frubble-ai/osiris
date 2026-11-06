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
