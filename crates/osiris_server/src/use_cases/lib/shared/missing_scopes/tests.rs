//! A missing scope is named; a granted, a covering, or an unscoped token passes.

use super::{missing_scopes, mocks::nesting_provider::nesting};
use rstest::rstest;

#[rstest]
#[case::granted(Some("read,write"), &["read"], &[])]
#[case::spaces(Some("read write"), &["write"], &[])]
#[case::missing(Some("read"), &["read", "write"], &["write"])]
#[case::covered_by_a_wider_scope(Some("admin"), &["admin:users", "admin:teams"], &[])]
#[case::a_narrower_scope_covers_nothing_wider(Some("admin:users"), &["admin"], &["admin"])]
#[case::no_scopes_recorded(None, &["read"], &[])]
#[case::a_token_with_no_scopes(Some(""), &["read"], &[])]
fn names_the_scopes_a_connection_lacks(
    #[case] granted: Option<&str>,
    #[case] required: &[&str],
    #[case] missing: &[&str],
) {
    assert_eq!(missing_scopes(&nesting(), granted, required), missing);
}
