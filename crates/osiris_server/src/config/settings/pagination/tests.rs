//! The page length for a request's `limit`.

use super::PaginationSettings;
use rstest::rstest;

#[rstest]
#[case::none_asked(None, 20)]
#[case::zero(Some(0), 20)]
#[case::negative(Some(-5), 20)]
#[case::in_range(Some(50), 50)]
#[case::the_longest_allowed(Some(100), 100)]
#[case::above_the_longest(Some(500), 100)]
fn bounds_the_asked_length(#[case] asked: Option<i64>, #[case] expected: i64) {
    assert_eq!(PaginationSettings::default().limit(asked), expected);
}
