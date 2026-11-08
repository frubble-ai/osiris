//! A path with a dot-segment is refused, however it is written; any other path is kept as it is.

use super::request_path;
use crate::ProviderError;
use rstest::rstest;

#[rstest]
#[case::parent("/repos/o/r/contents/../../../../user")]
#[case::current("/repos/o/r/./hooks")]
#[case::trailing_parent("/repos/o/r/contents/..")]
#[case::encoded("/repos/o/r/contents/%2e%2e")]
#[case::half_encoded("/repos/o/r/contents/.%2E")]
#[case::relative("../user")]
fn refuses_a_dot_segment(#[case] path: &str) {
    assert!(matches!(
        request_path("acme", path),
        Err(ProviderError::Invalid { .. })
    ));
}

#[rstest]
#[case::plain("/repos/o/r")]
#[case::dotted_names("/repos/o.o/r..r/contents/.github/a.b")]
#[case::relative_method("conversations.list")]
#[case::three_dots("/repos/o/r/contents/...")]
fn keeps_any_other_path(#[case] path: &str) {
    assert_eq!(request_path("acme", path).unwrap(), path);
}
