//! Tests of `refresh`.

use super::*;

#[test]
fn refreshes_the_connection_by_id() {
    assert_eq!(
        Refresh { id: "ca_1".into() }.path(),
        "/v1/connected-accounts/ca_1/refresh"
    );
}
