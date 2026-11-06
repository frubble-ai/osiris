//! Tests of `first_refresh`.

use super::*;

#[test]
fn requires_a_refresh_token_only_from_a_provider_that_issues_them() {
    assert_eq!(first_refresh(true, true), FirstRefresh::Schedule);
    assert_eq!(
        first_refresh(true, false),
        FirstRefresh::MissingRefreshToken
    );
    assert_eq!(first_refresh(false, false), FirstRefresh::NotNeeded);
    assert_eq!(first_refresh(false, true), FirstRefresh::Schedule);
}
