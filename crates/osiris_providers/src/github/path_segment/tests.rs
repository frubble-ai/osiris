//! A path segment can't carry a slash, a query or a fragment into the path.

use super::path_segment;
use rstest::rstest;

#[rstest]
#[case::plain("octo-cat_1.0~x", "octo-cat_1.0~x")]
#[case::a_slash("feature/login", "feature%2Flogin")]
#[case::a_query("a?b#c", "a%3Fb%23c")]
#[case::a_space_and_unicode("é x", "%C3%A9%20x")]
fn encodes_what_would_change_the_path(#[case] value: &str, #[case] encoded: &str) {
    assert_eq!(path_segment(value), encoded);
}
